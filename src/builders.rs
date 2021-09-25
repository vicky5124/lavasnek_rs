use crate::error;
use crate::events;
use crate::model::TrackQueue;
use crate::Lavalink;

use pyo3::prelude::*;

use lavalink_rs::{
    builders::{LavalinkClientBuilder, PlayParameters},
    error::LavalinkError,
    model::TrackQueue as LavaTrackQueue,
    LavalinkClient,
};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

/// __new__()
///
/// All of the methods that return Self also modify Self, so you can chain them, or call the
/// individually.
///
/// Positional Arguments:
/// - `bot_id` : `Unsigned 64 bit integer`
/// - `bot_token` : `String`
///
/// Returns: `Self`
#[pyclass]
#[pyo3(text_signature = "(bot_id, bot_token, /)")]
pub struct LavalinkBuilder {
    pub builder: LavalinkClientBuilder,
}

#[pyclass]
pub struct PlayBuilder {
    pub builder: PlayParameters,
}

#[pymethods]
impl LavalinkBuilder {
    #[new]
    fn new(bot_id: u64, token: String) -> Self {
        let builder = LavalinkClient::builder(bot_id, &token);

        Self { builder }
    }

    /// Uses the Self data to build a Lavalink client and return it.
    ///
    /// Can raise an exception if it's unable to connect to the lavalink server, discord server, or
    /// both.
    ///
    /// Positional Arguments:
    /// - `event_handler` : `impl LavalinkEventHandler`
    ///
    /// Returns: `Future<Result<Lavalink, builtins.ConnectionError>>`
    #[pyo3(text_signature = "($self, event_handler, /)")]
    fn build<'a>(&self, py: Python<'a>, event_handler: PyObject) -> PyResult<&'a PyAny> {
        let builder = self.builder.clone();
        let current_loop = pyo3_asyncio::get_running_loop(py)?;
        let loop_ref = PyObject::from(current_loop);

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let lava = builder
                .build(events::LavalinkEventHandler {
                    inner: event_handler,
                    current_loop: loop_ref,
                })
                .await
                .map_err(|e| error::ConnectionError::new_err(e.to_string()))?;
            let lavalink = Lavalink { lava };

            Ok(Python::with_gil(|py| lavalink.into_py(py)))
        })
    }

    /// Sets the host. (Default to: 127.0.0.1)
    ///
    /// Positional Arguments:
    /// - `host` : `String`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, host, /)")]
    fn set_host(mut slf: PyRefMut<Self>, host: String) -> PyRefMut<Self> {
        slf.builder.host = host;
        slf
    }

    /// Sets the port. (Default to: 2333)
    ///
    /// Positional Arguments:
    /// - `port` : `Unsigned 16 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, port, /)")]
    fn set_port(mut slf: PyRefMut<Self>, port: u16) -> PyRefMut<Self> {
        slf.builder.port = port;
        slf
    }

    /// Sets the host and port from an address.
    ///
    /// Can raise an exception if the address is an invalid IPv4 or IPv6.
    ///
    /// Positional Arguments:
    /// - `address` : `String`
    ///
    /// Returns: `Result<Self, ipaddress.AddressValueError>`
    #[pyo3(text_signature = "($self, address, /)")]
    fn set_addr(mut slf: PyRefMut<Self>, addr: String) -> PyResult<PyRefMut<Self>> {
        let addr = SocketAddr::from_str(&addr)
            .map_err(|e| error::AddressValueError::new_err(e.to_string()))?;

        slf.builder.host = addr.ip().to_string();
        slf.builder.port = addr.port();

        Ok(slf)
    }

    /// Sets the lavalink password. (Default to: "youshallnotpass")
    ///
    /// Positional Arguments:
    /// - `password` : `String`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, password, /)")]
    fn set_password(mut slf: PyRefMut<Self>, password: String) -> PyRefMut<Self> {
        slf.builder.password = password;
        slf
    }

    /// Sets the number of shards. (Default to: 1)
    ///
    /// Positional Arguments:
    /// - `shard_count` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, shard_count, /)")]
    fn set_shard_count(mut slf: PyRefMut<Self>, shard_count: u64) -> PyRefMut<Self> {
        slf.builder.shard_count = shard_count;
        slf
    }

    /// Sets the ID of the bot.
    ///
    /// Positional Arguments:
    /// - `bot_id` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, bot_id, /)")]
    fn set_bot_id(mut slf: PyRefMut<Self>, bot_id: u64) -> PyRefMut<Self> {
        slf.builder.bot_id = bot_id.into();
        slf
    }

    /// Sets the token of the bot.
    ///
    /// Positional Arguments:
    /// - `bot_token` : `String`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, bot_token, /)")]
    fn set_bot_token(mut slf: PyRefMut<Self>, bot_token: String) -> PyRefMut<Self> {
        slf.builder.bot_token = bot_token;
        slf
    }

    /// Sets if the lavalink server is behind SSL. (Default to: False)
    ///
    /// Positional Arguments:
    /// - `is_ssl` : `bool`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, is_ssl, /)")]
    fn set_is_ssl(mut slf: PyRefMut<Self>, is_ssl: bool) -> PyRefMut<Self> {
        slf.builder.is_ssl = is_ssl;
        slf
    }

    /// Sets if the discord gateway for voice connections should start or not. (Default to: True)
    ///
    /// Positional Arguments:
    /// - `start_gateway` : `bool`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, start_gateway, /)")]
    fn set_start_gateway(mut slf: PyRefMut<Self>, start_gateway: bool) -> PyRefMut<Self> {
        slf.builder.start_gateway = start_gateway;
        slf
    }

    /// Sets the time to wait before starting the first discord gateway connection. (Default to: 6
    /// seconds)
    ///
    /// Positional Arguments:
    /// - `time` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, time, /)")]
    fn set_gateway_start_wait_time_secs(mut slf: PyRefMut<Self>, time: u64) -> PyRefMut<Self> {
        slf.builder.gateway_start_wait_time = Duration::from_secs(time);
        slf
    }

    /// Sets the time to wait before starting the first discord gateway connection.
    ///
    /// Positional Arguments:
    /// - `time` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, time, /)")]
    fn set_gateway_start_wait_time_millis(mut slf: PyRefMut<Self>, time: u64) -> PyRefMut<Self> {
        slf.builder.gateway_start_wait_time = Duration::from_millis(time);
        slf
    }
}

#[pymethods]
impl PlayBuilder {
    // NOTE: this can only return a network error.
    /// Starts playing the track.
    ///
    /// Returns: `Future<Result<None, lavasnek_rs.NetworkError>>`
    #[pyo3(text_signature = "($self, /)")]
    fn start<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let builder = self.builder.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            builder
                .start()
                .await
                .map_err(|e| error::NetworkError::new_err(e.to_string()))?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    // NOTE: this can also return a network error.
    /// Adds the track to the node queue.
    ///
    /// If there's no queue loop running, this will start one up, and add it to the running loops
    /// on `Lavalink.loops()`
    ///
    /// Needs for `Lavalink.create_session() to be called first.
    ///
    /// Returns: `Future<Result<None, [lavasnek_rs.NoSessionPresent, lavasnek_rs.NetworkError]>>`
    #[pyo3(text_signature = "($self, /)")]
    fn queue<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let builder = self.builder.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            builder.queue().await.map_err(|e| match e {
                LavalinkError::NoSessionPresent => error::NoSessionPresent::new_err(e.to_string()),
                LavalinkError::ErrorWebsocketPayload(_) => {
                    error::NetworkError::new_err(e.to_string())
                }
                _ => error::Exception::new_err(e.to_string()),
            })?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Generates a TrackQueue from the builder.
    ///
    /// Returns: `TrackQueue`
    #[pyo3(text_signature = "($self, /)")]
    pub fn to_track_queue(&self) -> TrackQueue {
        let track_queue = LavaTrackQueue {
            track: self.builder.track.clone(),
            start_time: self.builder.start,
            end_time: if self.builder.finish == 0 {
                None
            } else {
                Some(self.builder.finish)
            },
            requester: self.builder.requester,
        };

        TrackQueue { inner: track_queue }
    }

    /// Sets the person that requested the song
    ///
    /// Positional Arguments:
    /// - `requester` : `Unsigned 64 bit integer` (User ID)
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, requester, /)")]
    fn requester(mut slf: PyRefMut<Self>, requester: u64) -> PyRefMut<Self> {
        slf.builder.requester = Some(requester.into());
        slf
    }

    /// Sets if the current playing track should be replaced with this new one.
    ///
    /// Positional Arguments:
    /// - `replace` : `bool`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, replace, /)")]
    fn replace(mut slf: PyRefMut<Self>, replace: bool) -> PyRefMut<Self> {
        slf.builder.replace = replace;
        slf
    }

    /// Sets the time the track will start at in seconds.
    ///
    /// Positional Arguments:
    /// - `start` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, start, /)")]
    fn start_time_secs(mut slf: PyRefMut<Self>, start: u64) -> PyRefMut<Self> {
        slf.builder.start = Duration::from_secs(start).as_millis() as u64;
        slf
    }

    /// Sets the time the track will finish at in seconds.
    ///
    /// Positional Arguments:
    /// - `finish` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, finish, /)")]
    fn finish_time_secs(mut slf: PyRefMut<Self>, finish: u64) -> PyRefMut<Self> {
        slf.builder.finish = Duration::from_secs(finish).as_millis() as u64;
        slf
    }

    /// Sets the time the track will start at in milliseconds.
    ///
    /// Positional Arguments:
    /// - `start` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, start, /)")]
    fn start_time_millis(mut slf: PyRefMut<Self>, start: u64) -> PyRefMut<Self> {
        slf.builder.start = start;
        slf
    }

    /// Sets the time the track will finish at in milliseconds.
    ///
    /// Positional Arguments:
    /// - `finish` : `Unsigned 64 bit integer`
    ///
    /// Returns: `Self`
    #[pyo3(text_signature = "($self, finish, /)")]
    fn finish_time_millis(mut slf: PyRefMut<Self>, finish: u64) -> PyRefMut<Self> {
        slf.builder.finish = finish;
        slf
    }
}
