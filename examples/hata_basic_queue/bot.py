import logging
import os
import sys

import hata

# Not accessed, but needed to run asyncio stuff.
from hata.ext import asyncio
from hata.ext.commands_v2 import checks

import lavasnek_rs

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)


PREFIX = ","
TOKEN = os.environ["DISCORD_TOKEN"]
LAVALINK_PASSWORD = os.environ["LAVALINK_PASSWORD"]

# If True connect to voice with the hata gateway instead of lavasnek_rs's
HATA_VOICE = False


class Data:
    """Global data shared across the entire bot, used to store dashboard values."""

    def __init__(self):
        self.lavalink: lavasnek_rs.Lavalink = None


class EventHandler:
    """Events from the Lavalink server"""

    async def track_start(self, _lava_client, event):
        logging.info("Track started on guild: %s", event.guild_id)

    async def track_finish(self, _lava_client, event):
        logging.info("Track finished on guild: %s", event.guild_id)

    async def track_exception(self, lavalink, event):
        logging.warning("Track exception event happened on guild: %d", event.guild_id)

        # If a track was unable to be played, skip it
        skip = await lavalink.skip(event.guild_id)
        node = await lavalink.get_guild_node(event.guild_id)

        if not skip:
            await event.message.respond("Nothing to skip")
        else:
            if not node.queue and not node.now_playing:
                await lavalink.stop(event.guild_id)


bot = hata.Client(TOKEN, extensions="commands_v2", prefix=PREFIX)
bot.data = Data()


async def _join(ctx) -> int:
    """Join the user's voice channel and create a lavalink session."""
    voice_state = ctx.voice_state
    if voice_state is None:
        await ctx.reply("You are not at a voice channel!")
        return 0

    channel = voice_state.channel

    try:
        if HATA_VOICE:
            await ctx.client.join_voice(voice_state.channel)
            connection_info = await ctx.client.data.lavalink.wait_for_full_connection_info_insert(ctx.guild.id)
        else:
            connection_info = await ctx.client.data.lavalink.join(ctx.guild.id, channel.id)
    except TimeoutError:
        await ctx.reply(
            "I was unable to connect to the voice channel, maybe missing permissions? or some internal issue."
        )
        return 0

    await ctx.client.data.lavalink.create_session(connection_info)

    return channel.id


@bot.commands
@checks.guild_only()
async def join(ctx):
    """Joins the voice channel you are in."""
    channel_id = await _join(ctx)

    if channel_id:
        await ctx.reply(f"Joined <#{channel_id}>")


@bot.commands
@checks.guild_only()
async def leave(ctx):
    """Leaves the voice channel the bot is in, clearing the queue."""
    if HATA_VOICE:
        voice_client = ctx.voice_client
        if voice_client is None:
            await ctx.reply("There is no voice client at your guild.")
            return

        await ctx.client.data.lavalink.destroy(ctx.guild.id)
        await voice_client.disconnect()
        await ctx.client.data.lavalink.wait_for_connection_info_remove(ctx.guild.id)
    else:
        await ctx.client.data.lavalink.leave(ctx.guild.id)

    # Destroy nor leave remove the node nor the queue loop, you should do this manually.
    await ctx.client.data.lavalink.remove_guild_node(ctx.guild.id)
    await ctx.client.data.lavalink.remove_guild_from_loops(ctx.guild.id)

    await ctx.reply("Left voice channel")


@bot.commands
@checks.guild_only()
async def play(ctx, query=None):
    """Searches the query on youtube, or adds the URL to the queue."""
    con = await bot.data.lavalink.get_guild_gateway_connection_info(ctx.guild.id)
    # Join the user's voice channel if the bot is not in one.
    if not con:
        await _join(ctx)

    # Search the query, auto_search will get the track from a url if possible, otherwise,
    # it will search the query on youtube.
    query_information = await bot.data.lavalink.auto_search_tracks(query)

    if not query_information.tracks:  # tracks is empty
        await ctx.reply("Could not find any video of the search query.")
        return

    try:
        # `.requester()` To add the requester, so you can show it on now-playing or queue.
        # `.queue()` To add the track to the queue rather than starting to play the track now.
        await ctx.client.data.lavalink.play(ctx.guild.id, query_information.tracks[0]).requester(ctx.author.id).queue()
    except lavasnek_rs.NoSessionPresent:
        await ctx.reply(f"Use `{PREFIX}join` first")
        return

    await ctx.reply(f"Added to queue: {query_information.tracks[0].info.title}")


@bot.commands
@checks.guild_only()
async def stop(ctx):
    """Stops the current song (skip to continue)."""
    await ctx.client.data.lavalink.stop(ctx.guild.id)
    await ctx.reply("Stopped playing")


@bot.commands
@checks.guild_only()
async def skip(ctx):
    """Skips the current song."""
    skip = await ctx.client.data.lavalink.skip(ctx.guild.id)
    node = await ctx.client.data.lavalink.get_guild_node(ctx.guild.id)

    if not skip:
        await ctx.reply("Nothing to skip")
    else:
        # If the queue is empty, the next track won't start playing (because isn't any),
        # so we stop the player.
        if not node.queue and not node.now_playing:
            await ctx.client.data.lavalink.stop(ctx.guild.id)

        await ctx.reply(f"Skipped: {skip.track.info.title}")


@bot.commands
@checks.guild_only()
async def pause(ctx):
    """Pauses the current song."""
    await ctx.client.data.lavalink.pause(ctx.guild.id)
    await ctx.reply("Paused player")


@bot.commands
@checks.guild_only()
async def resume(ctx):
    """Resumes playing the current song."""
    await ctx.client.data.lavalink.resume(ctx.guild.id)
    await ctx.reply("Resumed player")


@bot.commands(aliases=["np"])
@checks.guild_only()
async def now_playing(ctx):
    """Gets the song that's currently playing."""
    node = await ctx.client.data.lavalink.get_guild_node(ctx.guild.id)

    if not node or not node.now_playing:
        await ctx.reply("Nothing is playing at the moment.")
        return

    # For queue, iterate over `node.queue`, where index 0 is now_playing.
    await ctx.reply(f"Now Playing: {node.now_playing.track.info.title}")


@bot.commands
@checks.guild_only()
async def data(ctx, *args):
    """Load or read data from the node.

    If just `data` is ran, it will show the current data, but if `data <key> <value>` is ran, it
    will insert that data to the node and display it."""

    node = await ctx.client.data.lavalink.get_guild_node(ctx.guild.id)

    if not args:
        await ctx.respond(await node.get_data())
    else:
        if len(args) == 1:
            await node.set_data({args[0]: args[0]})
        else:
            await node.set_data({args[0]: args[1]})
        await ctx.respond(await node.get_data())


@bot.events
async def ready(client):
    """Event that triggers when the hata gateway is ready."""
    builder = (
        # TOKEN can be an empty string if you don't want to use lavasnek's discord gateway.
        lavasnek_rs.LavalinkBuilder(client.id, TOKEN)
        # This is the default value, so this is redundant, but it's here to show how to set a custom one.
        .set_host("127.0.0.1").set_password(LAVALINK_PASSWORD)
    )

    if HATA_VOICE:
        builder.set_start_gateway(False)

    lava_client = await builder.build(EventHandler())

    client.data.lavalink = lava_client

    logging.info("Bot is ready!")


if HATA_VOICE:

    @bot.events
    async def user_voice_update(client, event, _old):
        await client.data.lavalink.raw_handle_event_voice_state_update(
            event.guild.id,
            event.user.id,
            event.session_id,
            event.channel.id,
        )

    logging.error(
        "The `voice_server_update` event is not exposed by HATA, so the only way to use lavasnek_rs with it RN is to use the lavasnek discord gateway to connect."
    )
    sys.exit(0)

    # This is commented out because the `voice_server_update` event is not exposed by HATA, so the only way to use lavasnek_rs with it RN is to use the lavasnek discord gateway to connect.
    # @bot.events
    # async def voice_server_update(client, event):
    #    await client.data.lavalink.raw_handle_event_voice_server_update(event.guild_id, event.endpoint, event.token)


if __name__ == "__main__":
    bot.start()
