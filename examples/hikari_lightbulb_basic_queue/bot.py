import os

import hikari
import lightbulb
from consts import PREFIX, TOKEN

# You may want to enable ALL intents here
bot = lightbulb.BotApp(token=TOKEN, prefix=PREFIX)


@bot.listen()
async def starting_load_extensions(_: hikari.StartingEvent) -> None:
    """Load the music extension when Bot starts."""
    bot.load_extensions("music_plugin")


@bot.command()
@lightbulb.command("ping", "The bot's ping.")
@lightbulb.implements(lightbulb.PrefixCommand, lightbulb.SlashCommand)
async def ping(ctx: lightbulb.Context) -> None:
    """Typical Ping-Pong command"""
    await ctx.respond("Ping?")


if __name__ == "__main__":
    if os.name != "nt":
        import uvloop

        uvloop.install()

    bot.run()
