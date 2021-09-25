import os
import logging
from typing import Any, Union, List

import hikari
import lavasnek_rs

PREFIX = ","
TOKEN = os.environ["DISCORD_TOKEN"]

# If True connect to voice with the hikari gateway instead of lavasnek_rs's
HIKARI_VOICE = False


def is_command(cmd_name: str, content: str) -> bool:
    """Check if the message sent is a valid command."""
    return content.startswith(f"{PREFIX}{cmd_name}")


def get_args(cmd_name: str, content: str, is_text: bool) -> Union[str, List[str]]:
    """Return the arguments of a command."""
    if is_text:
        return content[len(f"{PREFIX}{cmd_name}") :].rstrip()
    else:
        return list(filter(lambda i: i, content[len(f"{PREFIX}{cmd_name}") :].split()))


class Data:
    """Global data shared across the entire bot, used to store dashboard values."""

    def __init__(self) -> None:
        self.lavalink: lavasnek_rs.Lavalink = None


class Bot(hikari.GatewayBot):
    """Just implementing the data to the Bot."""

    def __init__(self, **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self.data = Data()


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


bot = Bot(token=TOKEN)


async def _join(event: hikari.GuildMessageCreateEvent) -> int:
    """Join the user's voice channel and create a lavalink session."""

    states = bot.cache.get_voice_states_view_for_guild(event.get_guild())
    voice_state = list(
        filter(lambda i: i.user_id == event.author_id, states.iterator())
    )

    if not voice_state:
        await event.message.respond("Connect to a voice channel first")
        return 0

    channel_id = voice_state[0].channel_id

    if HIKARI_VOICE:
        await bot.update_voice_state(event.guild_id, channel_id, self_deaf=True)
        connection_info = await bot.data.lavalink.wait_for_full_connection_info_insert(
            event.guild_id
        )
    else:
        try:
            connection_info = await bot.data.lavalink.join(event.guild_id, channel_id)
        except TimeoutError:
            await event.message.respond("I was unable to connect to the voice channel, maybe missing permissions? or some internal issue.")
            return 0

    await bot.data.lavalink.create_session(connection_info)

    return channel_id


@bot.listen()
async def on_message(event: hikari.GuildMessageCreateEvent) -> None:
    """Event that triggers on every new message."""

    if event.is_bot or not event.content or not event.guild_id:
        return

    if event.content.startswith(PREFIX):
        # "Typical" Ping-Pong command
        if is_command("ping", event.content):
            await event.message.respond("Ping?")

        # Lists all the commands
        elif is_command("help", event.content):
            await event.message.respond(
                "ping, join, leave, play <query>, stop, skip, pause, resume, data <key?> <value?>"
            )

        # Joins the voice channel the user is in.
        elif is_command("join", event.content):
            channel_id = await _join(event)

            if channel_id:
                await event.message.respond(f"Joined <#{channel_id}>")

        # Leaves the voice channel.
        elif is_command("leave", event.content):
            await bot.data.lavalink.destroy(event.guild_id)

            if HIKARI_VOICE:
                await bot.update_voice_state(event.guild_id, None)
                await bot.data.lavalink.wait_for_connection_info_remove(event.guild_id)
            else:
                await bot.data.lavalink.leave(event.guild_id)

            # Destroy nor leave remove the node nor the queue loop, you should do this manually.
            await bot.data.lavalink.remove_guild_node(event.guild_id)
            await bot.data.lavalink.remove_guild_from_loops(event.guild_id)

            await event.message.respond("Left voice channel")

        # Searches and adds a track to the queue.
        elif is_command("play", event.content):
            con = await bot.data.lavalink.get_guild_gateway_connection_info(
                event.guild_id
            )
            # Join the user's voice channel if the bot is not in one.
            if not con:
                await _join(event)

            args = get_args("play", event.content, False)
            query = " ".join(args)

            # Search the query, auto_search will get the track from a url if possible, otherwise,
            # it will search the query on youtube.
            query_information = await bot.data.lavalink.auto_search_tracks(query)

            if not query_information.tracks:  # tracks is empty
                await event.message.respond(
                    "Could not find any video of the search query."
                )
                return

            try:
                # `.requester()` To set who requested the track, so you can show it on now-playing or queue.
                # `.queue()` To add the track to the queue rather than starting to play the track now.
                await bot.data.lavalink.play(
                    event.guild_id, query_information.tracks[0]
                ).requester(event.author_id).queue()
            except lavasnek_rs.NoSessionPresent:
                await event.message.respond(f"Use `{PREFIX}join` first")
                return

            await event.message.respond(
                f"Added to queue: {query_information.tracks[0].info.title}"
            )

        # Stops the current song (skip to continue).
        elif is_command("stop", event.content):
            await bot.data.lavalink.stop(event.guild_id)
            await event.message.respond("Stopped playing")

        # Skips the current song.
        elif is_command("skip", event.content):
            skip = await bot.data.lavalink.skip(event.guild_id)
            node = await bot.data.lavalink.get_guild_node(event.guild_id)

            if not skip:
                await event.message.respond("Nothing to skip")
            else:
                # If the queue is empty, the next track won't start playing (because there isn't any),
                # so we stop the player.
                if not node.queue and not node.now_playing:
                    await bot.data.lavalink.stop(event.guild_id)

                await event.message.respond(f"Skipped: {skip.track.info.title}")

        # Pauses the current song.
        elif is_command("pause", event.content):
            await bot.data.lavalink.pause(event.guild_id)
            await event.message.respond("Paused player")

        # Resumes playing the current song.
        elif is_command("resume", event.content):
            await bot.data.lavalink.resume(event.guild_id)
            await event.message.respond("Resumed player")

        # Resume playing the current song.
        elif is_command("now_plaing", event.content) or is_command("np", event.content):
            node = await bot.data.lavalink.get_guild_node(event.guild_id)

            if not node or not node.now_playing:
                await event.message.respond("Nothing is playing at the moment.")
                return

            # for queue, iterate over `node.queue`, where index 0 is now_playing.
            await event.message.respond(f"Now Playing: {node.now_playing.track.info.title}")

        # Load or read data from the node.
        #
        # if just `data` is ran, it will show the current data, but if `data <key> <value>` is ran, it
        # will insert that data to the node and display it.
        elif is_command("data", event.content):
            args = get_args("data", event.content, False)
            node = await bot.data.lavalink.get_guild_node(event.guild_id)

            if not args:
                await event.message.respond(await node.get_data())
            else:
                if len(args) == 1:
                    await node.set_data({args[0]: args[0]})
                else:
                    await node.set_data({args[0]: args[1]})
                await event.message.respond(await node.get_data())


@bot.listen()
async def on_ready(event: hikari.ShardReadyEvent) -> None:
    """Event that triggers when the hikari gateway is ready."""

    builder = (
        # TOKEN can be an empty string if you don't want to use lavasnek's discord gateway.
        lavasnek_rs.LavalinkBuilder(event.my_user.id, TOKEN)
        # This is the default value, so this is redundant, but it's here to show how to set a custom one.
        .set_host("127.0.0.1")
        .set_password(os.environ["LAVALINK_PASSWORD"])
    )

    if HIKARI_VOICE:
        builder.set_start_gateway(False)

    lava_client = await builder.build(EventHandler())

    bot.data.lavalink = lava_client


if HIKARI_VOICE:

    @bot.listen()
    async def voice_state_update(event: hikari.VoiceStateUpdateEvent) -> None:
        await bot.data.lavalink.raw_handle_event_voice_state_update(
            event.state.guild_id,
            event.state.user_id,
            event.state.session_id,
            event.state.channel_id,
        )

    @bot.listen()
    async def voice_server_update(event: hikari.VoiceServerUpdateEvent) -> None:
        await bot.data.lavalink.raw_handle_event_voice_server_update(
            event.guild_id, event.endpoint, event.token
        )


bot.run()
