#[macro_use]
extern crate log;

mod builders;
mod error;
use builders::*;

use lavalink_rs::model::{ConnectionInfo as LavaConnectionInfo, Track};
use lavalink_rs::LavalinkClient;

use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

use tokio::time::{sleep, Duration};

/// This is never actually used, a dictionary is used instead. If you use a 3rd party method of
/// joining a voice channel, you can get this values from the `VOICE_STATE_UPDATE` and
/// `VOICE_SERVER_UPDATE` events, and manually build a dict with them.
///
/// Fields:
///
/// - `guild_id` : `Unsigned 64 bit integer`
/// - `channel_id` : `Unsigned 64 bit integer`
/// - `endpoint` : `String`
/// - `token` : `String`
/// - `session_id` : `String`
#[pyclass]
struct ConnectionInfo;

#[pyclass]
pub(crate) struct Lavalink {
    lava: LavalinkClient,
}

#[pymethods]
impl Lavalink {
    /// Joins a guild's voice channel using the lavalink-rs discord gateway.
    ///
    /// Returns information about the gateway connection, which can be used with `create_session()`
    /// to connect lavalink to that voice connection.
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
        let connection_info: LavaConnectionInfo = depythonize(connection_info.as_ref(py)).unwrap();

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
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, builtins.Exception>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn destroy<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .destroy(guild_id)
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Returns the Play builder.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    /// - `track` : `Track` - From the track search methods, it's a value from the "tracks" field.
    ///
    /// Returns: `PlayBuilder`
    #[pyo3(text_signature = "($self, track, /)")]
    fn play(&self, py: Python, guild_id: u64, track: PyObject) -> PlayBuilder {
        let track: Track = depythonize(track.as_ref(py)).unwrap();
        PlayBuilder {
            builder: self.lava.play(guild_id, track),
        }
    }

    /// Returns the tracks from the URL or query provided.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `query` : `String`
    ///
    /// Returns: `Future<Result<Tracks, builtins.Exception>>`
    #[pyo3(text_signature = "($self, query, /)")]
    fn get_tracks<'a>(&self, py: Python<'a>, query: String) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let tracks = lava_client
                .get_tracks(query)
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| pythonize(py, &tracks).unwrap()))
        })
    }

    /// Will automatically search the query on youtube if it's not a valid URL.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `query` : `String`
    ///
    /// Returns: `Future<Result<Tracks, builtins.Exception>>`
    #[pyo3(text_signature = "($self, query, /)")]
    fn auto_search_tracks<'a>(&self, py: Python<'a>, query: String) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let tracks = lava_client
                .auto_search_tracks(query)
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| pythonize(py, &tracks).unwrap()))
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
    /// Returns: `Future<Result<Tracks, builtins.Exception>>`
    #[pyo3(text_signature = "($self, query, /)")]
    fn search_tracks<'a>(&self, py: Python<'a>, query: String) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let tracks = lava_client
                .search_tracks(query)
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| pythonize(py, &tracks).unwrap()))
        })
    }

    /// Stops the current player.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<None, builtins.Exception>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn stop<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .stop(guild_id)
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Skips the current playing track to the next item on the queue.
    ///
    /// If nothing is in the queue, the currently playing track will keep playing.
    /// Check if the queue is empty and run `stop()` if that's the case.
    ///
    /// This can raise an exception if a network error happens.
    ///
    /// Positional Arguments:
    /// - `guild_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Future<Result<Option<TrackQueue>, builtins.Exception>>`
    #[pyo3(text_signature = "($self, guild_id, /)")]
    fn skip<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let track = lava_client.skip(guild_id).await;
            // TrackQueue

            Ok(Python::with_gil(|py| {
                if let Some(track) = track {
                    pythonize(py, &track).unwrap()
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
    /// Returns: `Future<Result<None, builtins.Exception>>`
    #[pyo3(text_signature = "($self, guild_id, pause, /)")]
    fn set_pause<'a>(&self, py: Python<'a>, guild_id: u64, pause: bool) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .set_pause(guild_id, pause)
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

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
    /// Returns: `Future<Result<None, builtins.Exception>>`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn seek_secs<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .seek(guild_id, Duration::from_secs(time))
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

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
    /// Returns: `Future<Result<None, builtins.Exception>>`
    #[pyo3(text_signature = "($self, guild_id, time, /)")]
    fn seek_millis<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .seek(guild_id, Duration::from_millis(time))
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

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
    /// Returns: `Future<Result<None, builtins.Exception>>`
    #[pyo3(text_signature = "($self, guild_id, volume, /)")]
    fn volume<'a>(&self, py: Python<'a>, guild_id: u64, volume: u16) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .volume(guild_id, volume)
                .await
                .map_err(|e| error::Exception::new_err(e.to_string()))?;

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
            let loops = lava_client.loops().await;
            loops.remove(&guild_id);

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Get the current guild from the queue nodes.
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
                    pythonize(py, &node.to_owned()).unwrap()
                } else {
                    py.None()
                }
            }))
        })
    }

    /// Get the current guild from the queue nodes.
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
/// - If something returns a `Future<T>`, it means that it returns [this](https://docs.python.org/3/library/asyncio-future.html?#asyncio.Future),
/// and that function should be awaited to work.
/// - / on arguments means the end of positional arguments.
/// - Slef (with a capital S) means the type of self.
/// - For `Track`, `Tracks`, `TrackQueue` and `Node` documentation, check it out on
/// (docs.rs)[https://docs.rs/lavalink-rs]
#[pymodule]
fn lavasnek_rs(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(log_something, m)?)?;

    m.add_class::<Lavalink>()?;
    m.add_class::<LavalinkBuilder>()?;
    m.add_class::<PlayBuilder>()?;
    m.add_class::<ConnectionInfo>()?;

    m.add("NoSessionPresent", py.get_type::<error::NoSessionPresent>())?;

    Ok(())
}
