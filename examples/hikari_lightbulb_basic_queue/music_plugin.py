import logging
from typing import Any
from typing import Optional

import hikari
import lightbulb
from consts import LAVALINK_PASSWORD
from consts import PREFIX
from consts import TOKEN

import lavasnek_rs

# If True connect to voice with the hikari gateway instead of lavasnek_rs's
HIKARI_VOICE = False


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


class Music(lightbulb.Plugin):
    def __init__(self, bot: lightbulb.Bot) -> None:
        super().__init__()
        self.bot = bot

    async def _join(self, ctx: lightbulb.Context) -> Optional[hikari.Snowflake]:
        states = self.bot.cache.get_voice_states_view_for_guild(ctx.get_guild())
        voice_state = [state async for state in states.iterator().filter(lambda i: i.user_id == ctx.author.id)]

        if not voice_state:
            await ctx.respond("Connect to a voice channel first")
            return

        channel_id = voice_state[0].channel_id

        if HIKARI_VOICE:
            await self.bot.update_voice_state(ctx.guild_id, channel_id, self_deaf=True)
            connection_info = await self.bot.data.lavalink.wait_for_full_connection_info_insert(ctx.guild_id)
        else:
            try:
                connection_info = await self.bot.data.lavalink.join(ctx.guild_id, channel_id)
            except TimeoutError:
                await ctx.respond(
                    "I was unable to connect to the voice channel, maybe missing permissions? or some internal issue."
                )
                return

        await self.bot.data.lavalink.create_session(connection_info)

        return channel_id

    @lightbulb.listener(hikari.ShardReadyEvent)
    async def start_lavalink(self, _: hikari.ShardReadyEvent) -> None:
        """Event that triggers when the hikari gateway is ready."""

        builder = (
            # TOKEN can be an empty string if you don't want to use lavasnek's discord gateway.
            lavasnek_rs.LavalinkBuilder(self.bot.get_me().id, TOKEN)
            # This is the default value, so this is redundant, but it's here to show how to set a custom one.
            .set_host("127.0.0.1").set_password(LAVALINK_PASSWORD)
        )

        if HIKARI_VOICE:
            builder.set_start_gateway(False)

        lava_client = await builder.build(EventHandler())

        self.bot.data.lavalink = lava_client

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command()
    async def join(self, ctx: lightbulb.Context) -> None:
        """Joins the voice channel you are in."""
        channel_id = await self._join(ctx)

        if channel_id:
            await ctx.respond(f"Joined <#{channel_id}>")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command()
    async def leave(self, ctx: lightbulb.Context) -> None:
        """Leaves the voice channel the bot is in, clearing the queue."""

        await self.bot.data.lavalink.destroy(ctx.guild_id)

        if HIKARI_VOICE:
            await self.bot.update_voice_state(ctx.guild_id, None)
            await self.bot.data.lavalink.wait_for_connection_info_remove(ctx.guild_id)
        else:
            await self.bot.data.lavalink.leave(ctx.guild_id)

        # Destroy nor leave remove the node nor the queue loop, you should do this manually.
        await self.bot.data.lavalink.remove_guild_node(ctx.guild_id)
        await self.bot.data.lavalink.remove_guild_from_loops(ctx.guild_id)

        await ctx.respond("Left voice channel")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command()
    async def play(self, ctx: lightbulb.Context, *, query: str) -> None:
        """Searches the query on youtube, or adds the URL to the queue."""

        con = await self.bot.data.lavalink.get_guild_gateway_connection_info(ctx.guild_id)
        # Join the user's voice channel if the bot is not in one.
        if not con:
            await self._join(ctx)

        # Search the query, auto_search will get the track from a url if possible, otherwise,
        # it will search the query on youtube.
        query_information = await self.bot.data.lavalink.auto_search_tracks(query)

        if not query_information.tracks:  # tracks is empty
            await ctx.respond("Could not find any video of the search query.")
            return

        try:
            # `.requester()` To set who requested the track, so you can show it on now-playing or queue.
            # `.queue()` To add the track to the queue rather than starting to play the track now.
            await self.bot.data.lavalink.play(ctx.guild_id, query_information.tracks[0]).requester(
                ctx.author.id
            ).queue()
        except lavasnek_rs.NoSessionPresent:
            await ctx.respond(f"Use `{PREFIX}join` first")
            return

        await ctx.respond(f"Added to queue: {query_information.tracks[0].info.title}")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command()
    async def stop(self, ctx: lightbulb.Context) -> None:
        """Stops the current song (skip to continue)."""

        await self.bot.data.lavalink.stop(ctx.guild_id)
        await ctx.respond("Stopped playing")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command()
    async def skip(self, ctx: lightbulb.Context) -> None:
        """Skips the current song."""

        skip = await self.bot.data.lavalink.skip(ctx.guild_id)
        node = await self.bot.data.lavalink.get_guild_node(ctx.guild_id)

        if not skip:
            await ctx.respond("Nothing to skip")
        else:
            # If the queue is empty, the next track won't start playing (because there isn't any),
            # so we stop the player.
            if not node.queue and not node.now_playing:
                await self.bot.data.lavalink.stop(ctx.guild_id)

            await ctx.respond(f"Skipped: {skip.track.info.title}")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command()
    async def pause(self, ctx: lightbulb.Context) -> None:
        """Pauses the current song."""

        await self.bot.data.lavalink.pause(ctx.guild_id)
        await ctx.respond("Paused player")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command()
    async def resume(self, ctx: lightbulb.Context) -> None:
        """Resumes playing the current song."""

        await self.bot.data.lavalink.resume(ctx.guild_id)
        await ctx.respond("Resumed player")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.command(aliases=["np"])
    async def now_playing(self, ctx: lightbulb.Context) -> None:
        """Gets the song that's currently playing."""

        node = await self.bot.data.lavalink.get_guild_node(ctx.guild_id)

        if not node or not node.now_playing:
            await ctx.respond("Nothing is playing at the moment.")
            return

        # for queue, iterate over `node.queue`, where index 0 is now_playing.
        await ctx.respond(f"Now Playing: {node.now_playing.track.info.title}")

    @lightbulb.check(lightbulb.guild_only)
    @lightbulb.check(lightbulb.owner_only)  # Optional
    @lightbulb.command()
    async def data(self, ctx: lightbulb.Context, *args: Any) -> None:
        """Load or read data from the node.

        If just `data` is ran, it will show the current data, but if `data <key> <value>` is ran, it
        will insert that data to the node and display it."""

        node = await self.bot.data.lavalink.get_guild_node(ctx.guild_id)

        if not args:
            await ctx.respond(await node.get_data())
        else:
            if len(args) == 1:
                await node.set_data({args[0]: args[0]})
            else:
                await node.set_data({args[0]: args[1]})
            await ctx.respond(await node.get_data())

    if HIKARI_VOICE:

        @lightbulb.listener(hikari.VoiceStateUpdateEvent)
        async def voice_state_update(self, event: hikari.VoiceStateUpdateEvent) -> None:
            await self.bot.data.lavalink.raw_handle_event_voice_state_update(
                event.state.guild_id,
                event.state.user_id,
                event.state.session_id,
                event.state.channel_id,
            )

        @lightbulb.listener(hikari.VoiceServerUpdateEvent)
        async def voice_server_update(self, event: hikari.VoiceServerUpdateEvent) -> None:
            await self.bot.data.lavalink.raw_handle_event_voice_server_update(
                event.guild_id, event.endpoint, event.token
            )


def load(bot: lightbulb.Bot) -> None:
    bot.add_plugin(Music(bot))


def unload(bot: lightbulb.Bot) -> None:
    bot.remove_plugin("Music")
