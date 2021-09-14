#[macro_use]
extern crate log;

mod builders;
mod error;
use builders::*;

use lavalink_rs::model::{ConnectionInfo, Track};
use lavalink_rs::LavalinkClient;

use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

use tokio::time::{sleep, Duration};

#[pyclass]
pub(crate) struct Lavalink {
    lava: LavalinkClient,
}

#[pymethods]
impl Lavalink {
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

    fn create_session<'a>(&self, py: Python<'a>, connection_info: PyObject) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();
        let connection_info: ConnectionInfo = depythonize(connection_info.as_ref(py)).unwrap();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            lava_client
                .create_session(&connection_info)
                .await
                .map_err(|e| error::TypeError::new_err(format!("Missing field '{}'", e)))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

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

    fn play(&self, py: Python, guild_id: u64, track: PyObject) -> PlayBuilder {
        let track: Track = depythonize(track.as_ref(py)).unwrap();
        PlayBuilder {
            builder: self.lava.play(guild_id, track),
        }
    }

    /// Returns the tracks from the URL or query provided.
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
    fn pause<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        self.set_pause(py, guild_id, true)
    }

    /// Sets pause status to `False`
    fn resume<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        self.set_pause(py, guild_id, false)
    }

    /// Jumps to a specific time in the currently playing track.
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

    /// Alias to `seek()`
    fn jump_to_time_secs<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        time: u64,
    ) -> PyResult<&'a PyAny> {
        self.seek_secs(py, guild_id, time)
    }

    /// Alias to `seek()`
    fn scrub_secs<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        self.seek_secs(py, guild_id, time)
    }

    /// Jumps to a specific time in the currently playing track.
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

    /// Alias to `seek()`
    fn jump_to_time_millis<'a>(
        &self,
        py: Python<'a>,
        guild_id: u64,
        time: u64,
    ) -> PyResult<&'a PyAny> {
        self.seek_millis(py, guild_id, time)
    }

    /// Alias to `seek()`
    fn scrub_millis<'a>(&self, py: Python<'a>, guild_id: u64, time: u64) -> PyResult<&'a PyAny> {
        self.seek_millis(py, guild_id, time)
    }

    /// Sets the volume of the player.
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
    fn remove_guild_from_loops<'a>(&self, py: Python<'a>, guild_id: u64) -> PyResult<&'a PyAny> {
        let lava_client = self.lava.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let loops = lava_client.loops().await;
            loops.remove(&guild_id);

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Get the current guild from the queue nodes.
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

#[pyfunction]
fn rust_sleep(py: Python, seconds: u64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        sleep(Duration::from_secs(seconds)).await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pyfunction]
fn log_something() {
    info!("Something!");
}

#[pymodule]
fn lavasnek_rs(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(log_something, m)?)?;

    m.add_class::<Lavalink>()?;
    m.add_class::<LavalinkBuilder>()?;
    m.add_class::<PlayBuilder>()?;

    m.add("NoSessionPresent", py.get_type::<error::NoSessionPresent>())?;

    Ok(())
}
