import os
from typing import Any

import hikari
import lightbulb
from consts import PREFIX
from consts import TOKEN

import lavasnek_rs


class Data:
    """Global data shared across the entire bot, used to store dashboard values."""

    def __init__(self) -> None:
        self.lavalink: lavasnek_rs.Lavalink = None


class MusicBot(lightbulb.Bot):
    """Subclass of lightbulb's Bot object, used to store the lavalink client."""

    def __init__(self, *args: Any, **kwargs: Any):
        super().__init__(*args, **kwargs)
        self.data = Data()


# You may want to enable ALL intents here
bot = MusicBot(token=TOKEN, prefix=PREFIX)


@bot.listen()
async def starting_load_extensions(_: hikari.StartingEvent) -> None:
    """Load the music extension when Bot starts."""
    bot.load_extension("music_plugin")


@bot.command()
async def ping(ctx: lightbulb.Context) -> None:
    """Typical Ping-Pong command"""
    await ctx.respond("Ping?")


if __name__ == "__main__":
    if os.name != "nt":
        import uvloop

        uvloop.install()

    bot.run()
