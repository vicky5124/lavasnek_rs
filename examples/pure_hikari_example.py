import os

# import logging
from typing import Any

import hikari
import lavasnek_rs

PREFIX = ","
TOKEN = os.environ["DISCORD_TOKEN"]


def is_command(cmd_name: str, content: str) -> bool:
    """Check if the message sent is a valid command."""
    return content.startswith(f"{PREFIX}{cmd_name}")


def get_args(cmd_name: str, content: str) -> str:
    """Return the arguments of a command."""
    return content[len(f"{PREFIX}{cmd_name}") :]


class Data:
    """Global data shared across the entire bot, used to store dashboard values."""

    def __init__(self) -> None:
        self.lavalink: lavasnek_rs.Lavalink = None


class Bot(hikari.GatewayBot):
    """Just implementing the data to the Bot."""

    def __init__(self, **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self.data = Data()


bot = Bot(token=TOKEN)


async def _join(event: hikari.GuildMessageCreateEvent):
    """Join's the user's voice channel creating a lavalink session."""

    states = bot.cache.get_voice_states_view_for_guild(event.get_guild())
    voice_state = list(
        filter(lambda i: i.user_id == event.author_id, states.iterator())
    )

    if not voice_state:
        await event.message.respond("Connect to a voice channel first")
        return

    channel_id = voice_state[0].channel_id

    connection_info = await bot.data.lavalink.join(event.guild_id, channel_id)
    await bot.data.lavalink.create_session(connection_info)


@bot.listen()
async def on_message(event: hikari.GuildMessageCreateEvent) -> None:
    """Event that triggers on every new message."""

    if event.is_bot or not event.content or not event.guild_id:
        return

    if event.content.startswith(PREFIX):
        if is_command("ping", event.content):
            await event.message.respond("Ping?")

        elif is_command("help", event.content):
            await event.message.respond(
                "ping, join, leave, play, stop, skip, pause, resume"
            )

        elif is_command("join", event.content):
            await _join(event)

            await event.message.respond(f"Joined <#{channel_id}>")

        elif is_command("leave", event.content):
            await bot.data.lavalink.destroy(event.guild_id)
            await bot.data.lavalink.leave(event.guild_id)

            await event.message.respond("Left voice channel")

        elif is_command("play", event.content):
            con = await bot.data.lavalink.get_guild_gateway_connection_info(
                event.guild_id
            )
            if not con:
                await _join(event)

            args = get_args("play", event.content)

            query_information = await bot.data.lavalink.auto_search_tracks(args)

            if not query_information["tracks"]:  # tracks is empty
                await event.message.respond(
                    "Could not find any video of the search query."
                )
                return

            try:
                await bot.data.lavalink.play(
                    event.guild_id, query_information["tracks"][0]
                ).queue()
            except lavasnek_rs.NoSessionPresent:
                await event.message.respond(f"Use `{PREFIX}join` first")
                return

            await event.message.respond(
                f"Added to queue: {query_information['tracks'][0]['info']['title']}"
            )

        elif is_command("stop", event.content):
            await bot.data.lavalink.stop(event.guild_id)
            await bot.data.lavalink.remove_guild_from_loops(event.guild_id)
            await event.message.respond("Stopped playing")
        elif is_command("skip", event.content):
            skip = await bot.data.lavalink.skip(event.guild_id)
            node = await bot.data.lavalink.get_guild_node(event.guild_id)

            if not skip:
                await event.message.respond("Nothing to skip")
            else:
                if not node["queue"] and not node["now_playing"]:
                    await bot.data.lavalink.stop(event.guild_id)

                await event.message.respond(
                    f"Skipped: {skip['track']['info']['title']}"
                )

        elif is_command("pause", event.content):
            await bot.data.lavalink.pause(event.guild_id)
            await event.message.respond("Paused player")

        elif is_command("resume", event.content):
            await bot.data.lavalink.resume(event.guild_id)
            await event.message.respond("Resumed player")


@bot.listen()
async def on_ready(event: hikari.ShardReadyEvent) -> None:
    """Event that triggers when the hikari gateway is ready."""

    builder = (
        lavasnek_rs.LavalinkBuilder(event.my_user.id, os.environ["DISCORD_TOKEN"])
        .set_host("127.0.0.1")
        .set_password(os.environ["LAVALINK_PASSWORD"])
    )

    lava_client = await builder.build()

    bot.data.lavalink = lava_client


bot.run()
