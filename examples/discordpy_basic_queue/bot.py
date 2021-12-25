import logging
import os

import lavasnek_rs
import discord
from discord.ext import commands

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)

PREFIX = ","
TOKEN = os.environ["DISCORD_TOKEN"]
LAVALINK_PASSWORD = os.environ["LAVALINK_PASSWORD"]

# If True connect to voice with the discord.py's gateway instead of lavasnek_rs's
DPY_VOICE = True


class Data:
    """Global data shared across the entire bot, used to store dashboard values."""

    def __init__(self) -> None:
        self.lavalink: lavasnek_rs.Lavalink


class EventHandler:
    """Events from the Lavalink server"""

    async def track_start(self, _lavalink, event):
        logging.info("Track started on guild: %s", event.guild_id)

    async def track_finish(self, _lavalink, event):
        logging.info("Track finished on guild: %s", event.guild_id)

    async def track_exception(self, lavalink, event):
        logging.warning("Track exception event happened on guild: %d", event.guild_id)

        # If a track was unable to be played, skip it
        skip = await lavalink.skip(event.guild_id)
        node = await lavalink.get_guild_node(event.guild_id)

        if skip:
            if not node.queue and not node.now_playing:
                await lavalink.stop(event.guild_id)


bot = commands.Bot(
    command_prefix=commands.when_mentioned_or(PREFIX),
    intents=discord.Intents(guilds=True, guild_messages=True, voice_states=True),
)

bot.data = Data()  # not slotted, nice


class Music(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.command()
    async def join(self, ctx):
        """Joins the voice channel you are in."""

        await ctx.reply("Joined voice channel")

    @commands.command()
    async def leave(self, ctx):
        """Leaves the voice channel the bot is in, clearing the queue."""

        await ctx.bot.data.lavalink.destroy(ctx.guild.id)

        if DPY_VOICE:
            # await ctx.voice_bot.disconnect() # this needs PyNaCl fsr...
            ws = None

            if isinstance(ctx.bot, commands.AutoShardedBot):
                try:
                    ws = ctx.bot.shards[ctx.guild.shard_id].ws
                except AttributeError:
                    ws = ctx.bot.shards[ctx.guild.shard_id]._parent.ws

            if ctx.bot.shard_id is None or ctx.bot.shard_id == ctx.guild.shard_id:
                ws = ctx.bot.ws

            if ws:
                await ws.voice_state(ctx.guild.id, None)

                await ctx.bot.data.lavalink.wait_for_connection_info_remove(ctx.guild.id)
            else:
                await ctx.reply("Unknown error.")
                raise commands.CommandError("I don't know what the hell happened")

            await ctx.bot.data.lavalink.wait_for_connection_info_remove(ctx.guild.id)
        else:
            await ctx.bot.data.lavalink.leave(ctx.guild.id)

        # Destroy nor leave remove the node nor the queue loop, you should do this manually.
        await ctx.bot.data.lavalink.remove_guild_node(ctx.guild.id)
        await ctx.bot.data.lavalink.remove_guild_from_loops(ctx.guild.id)

        await ctx.reply("Left voice channel")

    @commands.command()
    async def play(self, ctx, *, query=None):
        """Searches the query on youtube, or adds the URL to the queue."""

        con = bot.data.lavalink.get_guild_gateway_connection_info(ctx.guild.id)
        # Join the user's voice channel if the bot is not in one.
        if not con:
            await ctx.reply("Connect to a voice channel or give me permissions to join it.")
            return

        # Search the query, auto_search will get the track from a url if possible, otherwise,
        # it will search the query on youtube.
        query_information = await bot.data.lavalink.auto_search_tracks(query)

        if not query_information.tracks:  # tracks is empty
            await ctx.reply("Could not find any video of the search query.")
            return

        try:
            # `.requester()` To add the requester, so you can show it on now-playing or queue.
            # `.queue()` To add the track to the queue rather than starting to play the track now.
            await ctx.bot.data.lavalink.play(ctx.guild.id, query_information.tracks[0]).requester(ctx.author.id).queue()
        except lavasnek_rs.NoSessionPresent:
            await ctx.reply(f"Use `{PREFIX}join` first")
            return

        await ctx.reply(f"Added to queue: {query_information.tracks[0].info.title}")

    @play.before_invoke
    @join.before_invoke
    async def ensure_voice(self, ctx):
        if ctx.author.voice:
            try:
                if DPY_VOICE:
                    # await ctx.author.voice.channel.connect() # this needs PyNaCl fsr...
                    ws = None

                    if isinstance(ctx.bot, commands.AutoShardedBot):
                        try:
                            ws = ctx.bot.shards[ctx.guild.shard_id].ws
                        except AttributeError:
                            ws = ctx.bot.shards[ctx.guild.shard_id]._parent.ws

                    if ctx.bot.shard_id is None or ctx.bot.shard_id == ctx.guild.shard_id:
                        ws = ctx.bot.ws

                    if ws:
                        await ws.voice_state(ctx.guild.id, str(ctx.author.voice.channel.id), self_deaf=True)

                        connection_info = await ctx.bot.data.lavalink.wait_for_full_connection_info_insert(ctx.guild.id)
                    else:
                        await ctx.reply("Unknown error.")
                        raise commands.CommandError("I don't know what the hell happened")
                else:
                    connection_info = await ctx.bot.data.lavalink.join(ctx.guild.id, ctx.author.voice.channel.id)
                await ctx.bot.data.lavalink.create_session(connection_info)
            except TimeoutError:
                await ctx.reply(
                    "You are not connected to a voice channel OR i didn't have permissions to join your voice channel."
                )
                raise commands.CommandError("Author not connected to a voice channel.")

        else:
            await ctx.reply("You are not connected to a voice channel.")

    if DPY_VOICE:

        @commands.Cog.listener()
        async def on_socket_response(self, data) -> None:
            if not data or "t" not in data:
                return

            if data["t"] == "VOICE_SERVER_UPDATE":
                guild_id = int(data["d"]["guild_id"])
                endpoint = data["d"]["endpoint"]
                token = data["d"]["token"]

                await self.bot.data.lavalink.raw_handle_event_voice_server_update(guild_id, endpoint, token)

            elif data["t"] == "VOICE_STATE_UPDATE":
                if not data["d"]["channel_id"]:
                    channel_id = None
                else:
                    channel_id = int(data["d"]["channel_id"])

                guild_id = int(data["d"]["guild_id"])
                user_id = int(data["d"]["user_id"])
                session_id = data["d"]["session_id"]

                self.bot.data.lavalink.raw_handle_event_voice_state_update(
                    guild_id,
                    user_id,
                    session_id,
                    channel_id,
                )


@bot.event
async def on_ready():
    """Event that triggers when the hata gateway is ready."""

    builder = (
        # TOKEN can be an empty string if you don't want to use lavasnek's discord gateway.
        lavasnek_rs.LavalinkBuilder(bot.user.id, TOKEN)
        # This is the default value, so this is redundant, but it's here to show how to set a custom one.
        .set_host("127.0.0.1").set_password(LAVALINK_PASSWORD)
    )

    if DPY_VOICE:
        builder.set_start_gateway(False)

    lava_bot = await builder.build(EventHandler())

    bot.data.lavalink = lava_bot

    logging.info(f"Logged in as {bot.user} (ID: {bot.user.id})")


if __name__ == "__main__":
    bot.add_cog(Music(bot))
    bot.run(TOKEN)
