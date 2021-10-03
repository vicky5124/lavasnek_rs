import typing

import hikari
import lavasnek_rs
import tanjun


music = tanjun.Component()


@music.with_slash_command
@tanjun.as_slash_command("join", "Connect the bot to a voice channel.")
async def join_as_slash(
    ctx: tanjun.abc.SlashContext,
) -> None:
    await _join_voice(ctx)


@music.with_message_command
@tanjun.as_message_command("join")
async def join_as_message(ctx: tanjun.abc.MessageContext) -> None:
    """Connect the bot to a voice channel."""
    await _join_voice(ctx)


async def _join_voice(ctx: tanjun.abc.Context) -> int:
    """Joins your voice channel."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    if ctx.client.cache and ctx.client.shards:
        # Get the users voice state if they didnt pass a channel
        if not (voice_state := ctx.client.cache.get_voice_state(ctx.guild_id, ctx.author)):
            await ctx.respond("Please connect to a voice channel.")
            return 1

        # Make sure our user is cached
        elif own_user := ctx.client.shards.get_me():
            # Check if there is a voice channel for the author
            if voice_state.channel_id and own_user.id in (
                # Check if we are in that channel
                ctx.client.cache.get_voice_states_view_for_channel(
                    ctx.guild_id,
                    voice_state.channel_id,
                )
            ):
                # We are in the channel already
                await ctx.respond(f"I am already connected to <#{voice_state.channel_id}>.")
                return 1

        # Bot joins voice
        await ctx.client.shards.update_voice_state(
            ctx.guild_id, voice_state.channel_id, self_deaf=True
        )
        # Lavasnek waits for the data on the event
        conn = await player.wait_for_full_connection_info_insert(ctx.guild_id)
        # Lavasnek tells lavalink to connect
        await player.create_session(conn)

        await ctx.respond(f"Connected to <#{voice_state.channel_id}>")
        return 0

    await ctx.respond("Unable to join voice. The cache is disabled or shards are down.")
    return 1


@music.with_slash_command
@tanjun.with_str_slash_option("song", "The title or youtube link of the song you want to play.")
@tanjun.as_slash_command("play", "Play a song, or add it to the queue.")
async def play_as_slash(ctx: tanjun.abc.SlashContext, song: str) -> None:
    await _play_track(ctx, song)


@music.with_message_command
@tanjun.with_greedy_argument("song")  # Set song to be greedy
@tanjun.with_parser  # Add an argument parser to the command
@tanjun.as_message_command("play")
async def play_as_message(ctx: tanjun.abc.MessageContext, song: str) -> None:
    """Play a song, or add it to the queue."""
    await _play_track(ctx, song)


async def _play_track(ctx: tanjun.abc.Context, song: str) -> None:
    """Attempts to play the song from youtube."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    # Check if we are connected to voice
    conn = await player.get_guild_gateway_connection_info(ctx.guild_id)

    if not conn:
        # Join the users voice channel if we are not already connected
        if await _join_voice(ctx):
            # Return out of the function if joining vc failed
            return

    if not (tracks := (await player.auto_search_tracks(song)).tracks):
        # We didnt find any tracks
        await ctx.respond(f"No tracks found found song: <{song}>")
        return

    try:
        # Play the first track in tracks
        # Set the requester, and queue the song
        await player.play(ctx.guild_id, tracks[0]).requester(ctx.author.id).queue()
    except lavasnek_rs.NoSessionPresent:
        # This shouldnt really ever happen
        await ctx.respond("Unable to join voice. This may be an internal error.")
        return

    await ctx.respond(f"Added to queue: `{tracks[0].info.title}`")


@music.with_slash_command
@tanjun.as_slash_command("leave", "Leaves the voice channel and clears the queue.")
async def leave_as_slash(ctx: tanjun.abc.SlashContext) -> None:
    await _leave_voice(ctx)


@music.with_message_command
@tanjun.as_message_command("leave")
async def leave_as_message(ctx: tanjun.abc.MessageContext) -> None:
    """Leaves the voice channel and clears the queue."""
    await _leave_voice(ctx)


async def _leave_voice(ctx: tanjun.abc.Context) -> None:
    """Stops playback of the current song."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    if await player.get_guild_gateway_connection_info(ctx.guild_id):
        # If were connected, destroy the connection
        await player.destroy(ctx.guild_id)

        if ctx.client.shards:
            # Set voice channel to None
            await ctx.client.shards.update_voice_state(ctx.guild_id, None)
            await player.wait_for_connection_info_remove(ctx.guild_id)

        # We must manually remove the node and queue loop from lavasnek
        await player.remove_guild_node(ctx.guild_id)
        await player.remove_guild_from_loops(ctx.guild_id)

        await ctx.respond("Disconnected from voice.")
        return

    await ctx.respond("I am not currently connected.")


@music.with_slash_command
@tanjun.as_slash_command("stop", "Stops the currently playing song, skip to play again.")
async def stop_as_slash(ctx: tanjun.abc.SlashContext) -> None:
    await _stop_playback(ctx)


@music.with_message_command
@tanjun.as_message_command("stop")
async def stop_as_message(ctx: tanjun.abc.MessageContext) -> None:
    """Stops the currently playing song, skip to play again."""
    await _stop_playback(ctx)


async def _stop_playback(ctx: tanjun.abc.Context) -> None:
    """Stops the currently playing song."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    await player.stop(ctx.guild_id)  # Stop the player
    await ctx.respond("Stopped playback.")


@music.with_slash_command
@tanjun.as_slash_command("skip", "Skips the current song.")
async def skip_as_slash(ctx: tanjun.abc.SlashContext) -> None:
    await _skip_track(ctx)


@music.with_message_command
@tanjun.as_message_command("skip")
async def skip_as_message(ctx: tanjun.abc.MessageContext) -> None:
    """Skips the current song."""
    await _skip_track(ctx)


async def _skip_track(ctx: tanjun.abc.Context) -> None:
    """Skips the current song."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    if not (skip := await player.skip(ctx.guild_id)):
        await ctx.respond("No tracks left to skip.")

    elif node := await player.get_guild_node(ctx.guild_id):
        # If we skipped and the queue is empty we need to
        # stop the player
        if not node.queue and not node.now_playing:
            await player.stop(ctx.guild_id)

        await ctx.respond(f"Skipped: {skip.track.info.title}")


@music.with_slash_command
@tanjun.as_slash_command("pause", "Pauses the current song.")
async def pause_as_slash(ctx: tanjun.abc.SlashContext) -> None:
    await _pause_playback(ctx)


@music.with_message_command
@tanjun.as_message_command("pause")
async def pause_as_message(ctx: tanjun.abc.MessageContext) -> None:
    """Pauses the current song."""
    await _pause_playback(ctx)


async def _pause_playback(ctx: tanjun.abc.Context) -> None:
    """Pauses the current song."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    if node := await player.get_guild_node(ctx.guild_id):
        # Use node data to check if we are paused
        if not (await node.get_data()).get("pause"):
            # If we are playing, pause.
            await node.set_data({"pause": True})
            await player.pause(ctx.guild_id)
            await ctx.respond("Paused playback.")
            return

        else:
            await ctx.respond("Playback is already paused.")
            return

    # We are not playing a track.
    await ctx.respond("No song to pause, try playing one.")


@music.with_slash_command
@tanjun.as_slash_command("resume", "Resumes the current song.")
async def resume_as_slash(ctx: tanjun.abc.SlashContext) -> None:
    await _resume_playback(ctx)


@music.with_message_command
@tanjun.as_message_command("resume")
async def resume_as_message(ctx: tanjun.abc.MessageContext) -> None:
    """Resumes the current song."""
    await _resume_playback(ctx)


async def _resume_playback(ctx: tanjun.abc.Context) -> None:
    """Resumes playing the current song."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    if node := await player.get_guild_node(ctx.guild_id):
        # Use node data to check if we are paused
        if (await node.get_data()).get("pause"):
            # We are paused, lets resume playback
            await node.set_data({"pause": False})
            await player.resume(ctx.guild_id)
            await ctx.respond("Resuming playback.")
            return

        else:
            # The track is already playing - do nothing
            return

    # We are not paused, no songs are playing
    await ctx.respond("No song to resume, try playing one.")


@music.with_slash_command
@tanjun.as_slash_command("playing", "Displays info on the currently playing song.")
async def playing_as_slash(ctx: tanjun.abc.Context) -> None:
    await _playing(ctx)


@music.with_message_command
@tanjun.as_message_command("playing")
async def playing_as_message(ctx: tanjun.abc.MessageContext) -> None:
    """Displays info on the currently playing song."""
    await _playing(ctx)


async def _playing(ctx: tanjun.abc.Context) -> None:
    """Displays info on the currently playing song."""
    player: lavasnek_rs.Lavalink = ctx.client.metadata["lavalink"]
    assert ctx.guild_id is not None

    if not (node := await player.get_guild_node(ctx.guild_id)):
        # No node, means no music
        await ctx.respond("Unable to connect to the node.")
        return

    if not node.now_playing:
        # Nothing is playing
        await ctx.respond("Nothing is playing now.")
        return

    if node.now_playing:
        # Info on the current track
        await ctx.respond(
            f"Title: {node.now_playing.track.info.title}\n"
            f"Requested by: <@!{node.queue[0].requester}>"
        )


@tanjun.as_loader
def load_component(client: tanjun.abc.Client) -> None:
    client.add_component(music.copy())
