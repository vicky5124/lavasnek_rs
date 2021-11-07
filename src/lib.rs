#[macro_use]
extern crate log;

mod builders;
mod error;
mod events;
mod model;

use builders::*;
use events::*;
use model::*;

use lavalink_rs::model::ConnectionInfo as LavaConnectionInfo;
use lavalink_rs::LavalinkClient;

use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

use tokio::time::{sleep, Duration};

#[pyclass]
#[derive(Clone)]
pub struct Lavalink {
    lava: LavalinkClient,
}

#[pymethods]
impl Lavalink {
    /// Start the discord gateway, if it has stopped, or it never started because the client builder was
    /// configured that way.
    ///
    /// If `wait_time` is passed, it will override the previosuly configured wait time.
    ///
    /// Positional Arguments:
    /// - `wait_time` : `Optional Unsigned 64 bit integer` -- seconds
    ///
    /// Returns: `Future<None>`
    #[pyo3(text_signature = "($self, /, wait_time)")]
    fn start_discord_gateway<'a>(
        &self,
        py: Python<'a>,
        wait_time: Option<u64>,
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .start_discord_gateway(wait_time.map(Duration::from_secs))
                .await;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Joins a guild's voice channel using the lavalink-rs discord gateway.
    ///
    /// Returns information about the gateway connection, which can be used with `create_session()`
    /// to connect lavalink to that voice connection.
    ///
    /// ```py
    /// lavalink: lavasnek_rs.Lavalink = ...
    ///
    /// connection_info = await lavalink.join(guild_id, voice_channel_id)
    /// await lavalink.create_session(connection_info)
    ///
    /// await send_message(f"Joined <#{voice_channel_id}>")
    /// ```
    ///
    /// Timing out means that there's either no permission to join the voice channel, or 5 seconds
    /// have happened since the function was called.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `channel_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<ConnectionInfo, builtins.TimeoutError>>`
    #[pyo3(text_signature = "($self, guild_id, channel_id, /)")]
    fn join<'a>(&self, py: Python<'a>, guild_id: u64, channel_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let connection_info = lava_client
                .join(guild_id, channel_id)
                .await
                .map_err(|e| error::TimeoutError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| {
                pythonize(py, &connection_info).unwrap()
            }))
        })
    }

    /// Leaves the current guild's voice channel using the lavalink-rs discord gateway.
    ///
    /// `Lavalink.destroy()` should be ran as well before this, to safely stop the lavalink session.
    /// ```py
    /// lavalink: lavasnek_rs.Lavalink = ...
    ///
    /// await lavalink.destroy(guild_id)
    /// await lavalink.leave(guild_id)
    ///
    /// await send_message("Left voice channel")
    /// ```
    ///
    /// Timing out means that 5 seconds have happened since the function was called.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, builtins.TimeoutError>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn leave<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .leave(guild_id)
                .await
                .map_err(|e| error::TimeoutError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Creates a session in Lavalink with a voice connection. This also creates a Node and inserts it.
    /// The node is not added to the loops unless `PlayBuilder.queue()` is ran.
    ///
    /// This can raise a TypeError if a necessary field of ConnectionInfo is missing.
    ///
    /// Positional Arguments:
    /// - `connection_info` : `ConnectionInfo` (obtained from `Lavalink.join()`)
    ///
    /// Returns: `Future<Result<None, builtins.TypeError>>`
    #[pyo3(text_signature = "($self, connection_info, /)")]
    fn create_session<'a>(&self, py: Python<'a>, connection_info: PyObject) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();
        let connection_info: LavaConnectionInfo = depythonize(connection_info.as_ref(py))?;

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .create_session(&connection_info)
                .await
                .map_err(|e| error::TypeError::new_err(format!("Missing field '{}'", e)))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Stops the session in Lavalink of the guild. This also creates a Node and inserts it.
    ///
    /// This method does not remove the guild from the running event loops, nor does it clear the Node,
    /// this allows for reconnecting without losing data. If you are having issues with disconnecting
    /// and reconnecting the bot to a voice channel, remove the guild from the running event loops and
    /// reset the nodes.
    ///
    /// ```py
    /// lavalink.remove_guild_node(guild_id)
    /// lavalink.remove_guild_from_loops(guild_id)
    /// ```
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn destroy<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .destroy(guild_id)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Returns the Play builder.
    ///
    /// ```py
    /// lavalink: lavasnek_rs.Lavalink = ...
    ///
    /// query_information = await lavalink.auto_search_tracks(search_query_or_url)
    ///
    /// if not query_information["tracks"]: # tracks is empty
    ///     await send_message("Could not find any video of the search query.")
    ///     return
    ///
    /// try:
    ///     await lavalink.play(
    ///         guild_id, query_information["tracks"][0]
    ///     ).requester(author_id).queue()
    /// except lavasnek_rs.NoSessionPresent:
    ///     await send_message(f"Use `{PREFIX}join` first")
    ///     return
    /// ```
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `track` : `Track` - From the track search methods, it's a value from the "tracks" field.
    ///
    /// Returns: `PlayBuilder`
    #[pyo3(text_signature = "($self, guild_id, track, /)")]
    fn play(&self, guild_id: u64, track: Track) -> PlayBuilder {
        PlayBuilder {
            builder: self.lava.play(guild_id, track.inner),
        }
    }

    /// Returns the tracks from the URL or query provided.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `query` : `String`
    ///
    /// Returns: `Future<Result<Tracks, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, query, /)")]
    fn get_tracks<'a>(&self, py: Python<'a>, query: String) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let tracks = lava_client
                .get_tracks(query)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| Tracks { inner: tracks }.into_py(py)))
        })
    }

    /// Will automatically search the query on youtube if it's not a valid URL.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `query` : `String`
    ///
    /// Returns: `Future<Result<Tracks, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, query, /)")]
    fn auto_search_tracks<'a>(&self, py: Python<'a>, query: String) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let tracks = lava_client
                .auto_search_tracks(query)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| Tracks { inner: tracks }.into_py(py)))
        })
    }

    /// Returns tracks from the search query.
    /// Uses youtube to search.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `query` : `String`
    ///
    /// Returns: `Future<Result<Tracks, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, query, /)")]
    fn search_tracks<'a>(&self, py: Python<'a>, query: String) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let tracks = lava_client
                .search_tracks(query)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| Tracks { inner: tracks }.into_py(py)))
        })
    }

    /// Returns information from a track.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `track` : `String` -- base 64
    ///
    /// Returns: `Future<Result<Info, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, track, /)")]
    fn decode_track<'a>(&self, py: Python<'a>, track: String) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let track_decode = lava_client
                .decode_track(track)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| {
                Info {
                    inner: track_decode,
                }
                .into_py(py)
            }))
        })
    }

    /// Stops the current player.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn stop<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .stop(guild_id)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Skips the current playing track to the next item on the queue.
    ///
    /// If nothing is in the queue, the currently playing track will keep playing.
    /// Check if the queue is empty and run `stop()` if that's the case.
    ///
    /// ```py
    /// lavalink: lavasnek_rs.Lavalink = ...
    ///
    /// skip = await lavalink.skip(guild_id)
    /// node = await lavalink.get_guild_node(guild_id)
    ///
    /// if not skip:
    ///     await send_message("Nothing to skip")
    /// else:
    ///     if not node["queue"] and not node["now_playing"]:
    ///         await lavalink.stop(guild_id)
    ///
    ///     await send_message(f"Skipped: {skip['track']['info']['title']}"
    /// ```
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Option<TrackQueue>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn skip<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let track = lava_client.skip(guild_id).await;
            // TrackQueue

            Ok(Python::with_gil(|py| {
                if let Some(track) = track {
                    TrackQueue { inner: track }.into_py(py)
                } else {
                    py.None()
                }
            }))
        })
    }

    /// Sets the pause status.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `pause` : `bool`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, pause, /)")]
    fn set_pause<'a>(&self, py: Python<'a>, guild_id: u64, pause: bool) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .set_pause(guild_id, pause)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Sets pause status to `True`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn pause<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        self.set_pause(py, guild_id, true)
    }

    /// Sets pause status to `False`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn resume<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        self.set_pause(py, guild_id, false)
    }

    /// Jumps to a specific time in the currently playing track.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `time` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn seek_secs<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .seek(guild_id, Duration::from_secs(time))
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Alias to `seek_secs()`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn jump_to_time_secs<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        time: u64,
    ) -> PyResult<&'a PyAny> {
        self.seek_secs(py, guild_id, time)
    }

    /// Alias to `seek_secs()`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn scrub_secs<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        self.seek_secs(py, guild_id, time)
    }

    /// Jumps to a specific time in the currently playing track.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `time` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn seek_millis<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .seek(guild_id, Duration::from_millis(time))
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Alias to `seek_millis()`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn jump_to_time_millis<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        time: u64,
    ) -> PyResult<&'a PyAny> {
        self.seek_millis(py, guild_id, time)
    }

    /// Alias to `seek_millis()`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn scrub_millis<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        self.seek_millis(py, guild_id, time)
    }

    /// Sets the volume of the player.
    /// Max is 1000, min is 0
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `Volume` : `Unsigned 16 bit integer`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, volume, /)")]
    fn volume<'a>(&self, py: Python<'a>, guild_id: u64, volume: u16) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .volume(guild_id, volume)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Sets all equalizer levels.
    ///
    /// - There are 15 bands (0-14) that can be changed.
    /// - The floating point value is the multiplier for the given band.
    /// - The default value is 0.
    /// - Valid values range from -0.25 to 1.0, where -0.25 means the given band is completely muted, and 0.25 means it is doubled.
    /// - Modifying the gain could also change the volume of the output.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `bands` : `List<64 bit floating point>` -- Must be 15 in length
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, bands, /)")]
    fn equalize_all<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        bands: [f64; 15],
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .equalize_all(guild_id, bands)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Equalize a dynamic set of bands, rather than just one or all of them at once.
    ///
    /// Unmentioned bands will remain unmodified.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `bands` : `List<64 bit floating point>` -- Must be 15 or less in length
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, bands, /)")]
    fn equalize_dynamic<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        bands: Vec<Band>,
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .equalize_dynamic(guild_id, bands.iter().map(|i| i.inner.clone()).collect())
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Equalizes a specific band.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `band` : `Band`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, band, /)")]
    fn equalize_band<'a>(&self, py: Python<'a>, guild_id: u64, band: Band) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .equalize_band(guild_id, band.inner)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Resets all equalizer levels.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn equalize_reset<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .equalize_reset(guild_id)
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Remove the guild from the queue loops.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<None>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn remove_guild_from_loops<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client.loops().await.remove(&guild_id);

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Remove the guild node.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<None>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn remove_guild_node<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client.nodes().await.remove(&guild_id);

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Get the current guild from the queue nodes.
    ///
    /// This returns a clone of the node, so modifying it won't change the real node, for that you
    /// will need to re-set the new node with `set_guild_node()`.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Option<Node>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn get_guild_node<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let nodes = lava_client.nodes().await;
            let node = nodes.get(&guild_id);

            Ok(Python::with_gil(|py| {
                if let Some(node) = node {
                    Node {
                        inner: node.clone(),
                    }
                    .into_py(py)
                } else {
                    py.None()
                }
            }))
        })
    }

    /// Set the node of a guild with a new one.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `node` : `Node`
    ///
    /// Returns: `Future<None>`
    #[pyo3(text_signature = "($self, guild_id, node, /)")]
    fn set_guild_node<'a>(&self, py: Python<'a>, guild_id: u64, node: Node) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let nodes = lava_client.nodes().await;
            nodes.insert(guild_id, node.inner);

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Get the current guild from the queue nodes.
    ///
    /// This returns a clone of the value, modifying it won't change it internally.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Option<ConnectionInfo>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn get_guild_gateway_connection_info<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let connections = lava_client.discord_gateway_connections().await;
            let connection = connections.get(&guild_id.into());

            Ok(Python::with_gil(|py| {
                if let Some(con) = connection {
                    pythonize(py, &con.to_owned()).unwrap()
                } else {
                    py.None()
                }
            }))
        })
    }

    /// Waits until the ConnectionInfo is complete and returns it.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `event_count` : `Unsigned 128 bit integer` defaults to 10
    ///
    /// Returns: `Future<ConnectionInfo>`
    #[pyo3(text_signature = "($self, guild_id, /, event_count=10)")]
    fn wait_for_full_connection_info_insert<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        event_count: Option<usize>,
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let connection_info = lavalink_rs::voice::wait_for_full_connection_info_insert(
                &lava_client,
                guild_id,
                event_count,
            )
            .await
            .map_err(|e| error::TimeoutError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| {
                pythonize(py, &connection_info).unwrap()
            }))
        })
    }

    /// Waits until the ConnectionInfo is removed.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `event_count` : `Unsigned 128 bit integer` defaults to 10
    ///
    /// Returns: `Future<None>`
    #[pyo3(text_signature = "($self, guild_id, /, event_count=10)")]
    fn wait_for_connection_info_remove<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        event_count: Option<usize>,
        //) -> LavalinkResult<()> {
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lavalink_rs::voice::wait_for_connection_info_remove(
                &lava_client,
                guild_id,
                event_count,
            )
            .await
            .map_err(|e| error::TimeoutError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Handles voice events to manage `ConnectionInfo` internally. This one is for the
    /// VOICE_SERVER_UPDATE event.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `endpint` : `String`
    /// - `token` : `String`
    ///
    /// Returns: `Future<None>`
    #[pyo3(text_signature = "($self, guild_id, endpoint, token, /)")]
    fn raw_handle_event_voice_server_update<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        endpoint: String,
        token: String,
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lavalink_rs::voice::raw_handle_event_voice_server_update(
                &lava_client,
                guild_id,
                endpoint,
                token,
            )
            .await;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Handles voice events to manage `ConnectionInfo` internally. This one is for the
    /// VOICE_STATE_UPDATE event.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `user_id` : `Unsigned 64 bit integer`
    /// - `session_id` : `String`
    /// - `channel_id` : `Optional Unsigned 64 bit integer`
    ///
    /// Returns: `Future<None>`
    #[pyo3(text_signature = "($self, guild_id, user_id, session_id, channel_id/)")]
    fn raw_handle_event_voice_state_update<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        user_id: u64,
        session_id: String,
        channel_id: Option<u64>,
    ) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lavalink_rs::voice::raw_handle_event_voice_state_update(
                &lava_client,
                guild_id,
                channel_id,
                user_id,
                session_id,
            )
            .await;

            Ok(Python::with_gil(|py| py.None()))
        })
    }
}

/// Test function, do not use.
#[pyfunction]
#[pyo3(text_signature = "(seconds, /)")]
fn rust_sleep(py: Python, seconds: u64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        sleep(Duration::from_secs(seconds)).await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

/// Test function, do not use.
#[pyfunction]
#[pyo3(text_signature = "()")]
fn log_something() {
    info!("Something!");
}

/// Cheat Sheet:
///
/// - Functions that return a `Result<T, E>` mean that it can raise an exception. `T` is the type they
/// return normally, and `E` is a list of possible exceptions that can raise.
/// - Functions that return an `Option<T>` mean that the value returned can be `None`, where `T` would be
/// the type of the returned value if not `None`.
/// - If something returns a `Future<T>`, it means that it returns
/// [this](https://docs.python.org/3/library/asyncio-future.html?#asyncio.Future),
/// and that function should be awaited to work.
/// - / on arguments means the end of positional arguments.
/// - Self (with a capital S) means the type of self.
/// - A type prefixed with `impl` means it's a Class that implements that Trait type.
#[pymodule]
fn lavasnek_rs(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(log_something, m)?)?;

    m.add_class::<Lavalink>()?;

    // builders
    m.add_class::<LavalinkBuilder>()?;
    m.add_class::<PlayBuilder>()?;

    // models
    m.add_class::<LavalinkEventHandler>()?;
    m.add_class::<ConnectionInfo>()?;
    m.add_class::<Track>()?;
    m.add_class::<Tracks>()?;
    m.add_class::<TrackQueue>()?;
    m.add_class::<Info>()?;
    m.add_class::<PlaylistInfo>()?;
    m.add_class::<Node>()?;
    m.add_class::<Band>()?;

    // event models
    m.add_class::<Stats>()?;
    m.add_class::<PlayerUpdate>()?;
    m.add_class::<TrackStart>()?;
    m.add_class::<TrackFinish>()?;
    m.add_class::<TrackException>()?;
    m.add_class::<TrackStuck>()?;
    m.add_class::<WebSocketClosed>()?;
    m.add_class::<PlayerDestroyed>()?;

    // exceptions
    m.add("NoSessionPresent", py.get_type::<error::NoSessionPresent>())?;
    m.add("NetworkError", py.get_type::<error::NetworkError>())?;

    Ok(())
}
