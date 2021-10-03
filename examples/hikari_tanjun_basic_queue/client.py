import os
import typing

import hikari
import tanjun
import lavasnek_rs


PREFIX = ","
TOKEN = os.environ["TOKEN"]


class EventHandler:
    """Handles events from the Lavalink server."""

    async def track_start(self, _: lavasnek_rs.Lavalink, event: lavasnek_rs.TrackStart) -> None:
        """Handles track start events."""
        print(f"Track started on guild: {event.guild_id}")

    async def track_finish(self, _: lavasnek_rs.Lavalink, event: lavasnek_rs.TrackFinish) -> None:
        """Handles track finish events."""
        print(f"Track finished on guild: {event.guild_id}")

    async def track_exception(
        self, lavalink: lavasnek_rs.Lavalink, event: lavasnek_rs.TrackException
    ) -> None:
        """Handles track exception events."""
        print(f"Track exception event happened on guild: {event.guild_id}")

        # If a track was unable to be played, skip it
        skip = await lavalink.skip(event.guild_id)
        node = await lavalink.get_guild_node(event.guild_id)

        if skip and node:
            if not node.queue and not node.now_playing:
                await lavalink.stop(event.guild_id)


class Client(tanjun.Client):
    def __init__(self, *args: typing.Any, **kwargs: typing.Any) -> None:
        super().__init__(*args, **kwargs)
        # Tanjun has a metadata property to store any general data
        # We will store the lavalink client there
        self.metadata["lavalink"] = None

        (  # Listen for voice and ready events
            self.add_listener(hikari.ShardReadyEvent, self.on_shard_ready)
            .add_listener(hikari.VoiceStateUpdateEvent, self.on_voice_state_update)
            .add_listener(hikari.VoiceServerUpdateEvent, self.on_voice_server_update)
        )

    async def on_shard_ready(self, event: hikari.ShardReadyEvent) -> None:
        """Event that triggers when the hikari gateway is ready."""
        builder = (
            lavasnek_rs.LavalinkBuilder(event.my_user.id, TOKEN)
            .set_host(os.environ["LAVALINK_HOST"])
            .set_password(os.environ["LAVALINK_PASSWORD"])
            .set_port(int(os.environ["LAVALINK_PORT"]))
            .set_start_gateway(False)
            # We set start gateway False because hikari can handle
            # voice events for us.
        )

        self.metadata["lavalink"] = await builder.build(EventHandler)

    async def on_voice_state_update(self, event: hikari.VoiceStateUpdateEvent) -> None:
        """Passes voice state updates to lavalink."""
        if self.metadata["lavalink"]:
            await self.metadata["lavalink"].raw_handle_event_voice_state_update(
                event.state.guild_id,
                event.state.user_id,
                event.state.session_id,
                event.state.channel_id,
            )

    async def on_voice_server_update(self, event: hikari.VoiceServerUpdateEvent) -> None:
        """Passes voice server updates to lavalink."""
        if self.metadata["lavalink"]:
            await self.metadata["lavalink"].raw_handle_event_voice_server_update(
                event.guild_id,
                event.endpoint,
                event.token,
            )


client = (
    Client.from_gateway_bot(
        bot := hikari.GatewayBot(token=TOKEN),
        mention_prefix=True,
    )
    # Adds our message command prefix
    .add_prefix(PREFIX)
    # Only allow commands in guilds
    .add_check(lambda ctx: ctx.guild_id is not None)
    # Load the music module
    .load_modules("music")
)


if __name__ == "__main__":
    if os.name != "nt":
        import uvloop

        uvloop.install()

    bot.run()
