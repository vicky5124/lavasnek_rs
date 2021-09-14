use crate::error;
use crate::Lavalink;

use pyo3::prelude::*;

use lavalink_rs::{
    async_trait,
    builders::{LavalinkClientBuilder, PlayParameters},
    gateway::LavalinkEventHandler,
    LavalinkClient,
};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

struct LavalinkHandler;

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {}

#[pyclass]
pub(crate) struct LavalinkBuilder {
    builder: LavalinkClientBuilder,
}

#[pyclass]
pub(crate) struct PlayBuilder {
    pub(crate) builder: PlayParameters,
}

#[pymethods]
impl LavalinkBuilder {
    #[new]
    fn new(bot_id: u64, token: String) -> Self {
        let builder = LavalinkClient::builder(bot_id, &token);

        Self { builder }
    }

    /// Sets the host.
    fn set_host(mut slf: PyRefMut<Self>, host: String) -> PyRefMut<Self> {
        slf.builder.host = host;
        slf
    }

    /// Sets the port.
    fn set_port(mut slf: PyRefMut<Self>, port: u16) -> PyRefMut<Self> {
        slf.builder.port = port;
        slf
    }

    /// Sets the host and port from an address.
    fn set_addr(mut slf: PyRefMut<Self>, addr: String) -> PyResult<PyRefMut<Self>> {
        let addr = SocketAddr::from_str(&addr)
            .map_err(|e| error::AddressValueError::new_err(e.to_string()))?;

        slf.builder.host = addr.ip().to_string();
        slf.builder.port = addr.port();

        Ok(slf)
    }

    /// Sets the number of shards.
    fn set_shard_count(mut slf: PyRefMut<Self>, shard_count: u64) -> PyRefMut<Self> {
        slf.builder.shard_count = shard_count;
        slf
    }

    /// Sets the ID of the bot.
    fn set_bot_id(mut slf: PyRefMut<Self>, bot_id: u64) -> PyRefMut<Self> {
        slf.builder.bot_id = bot_id.into();
        slf
    }

    /// Sets the token of the bot.
    fn set_bot_token(mut slf: PyRefMut<Self>, bot_token: String) -> PyRefMut<Self> {
        slf.builder.bot_token = bot_token;
        slf
    }

    /// Sets if the lavalink server is behind SSL
    fn set_is_ssl(mut slf: PyRefMut<Self>, is_ssl: bool) -> PyRefMut<Self> {
        slf.builder.is_ssl = is_ssl;
        slf
    }

    /// Sets the lavalink password.
    fn set_password(mut slf: PyRefMut<Self>, password: String) -> PyRefMut<Self> {
        slf.builder.password = password;
        slf
    }

    fn build<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let builder = self.builder.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let lava = builder
                .build(LavalinkHandler)
                .await
                .map_err(|e| error::ConnectionError::new_err(e.to_string()))?;
            let lavalink = Lavalink { lava };

            Ok(Python::with_gil(|py| lavalink.into_py(py)))
        })
    }
}

#[pymethods]
impl PlayBuilder {
    /// Starts playing the track.
    fn start<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let builder = self.builder.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            builder
                .start()
                .await
                .map_err(|e| error::NoSessionPresent::new_err(e.to_string()))?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Adds the track to the node queue.
    ///
    /// If there's no queue loop running, this will start one up, and add it to the running loops
    /// on `Lavalink.loops()`
    ///
    /// Needs for `Lavalink.create_session() to be called first.
    fn queue<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let builder = self.builder.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            builder
                .queue()
                .await
                .map_err(|e| error::NoSessionPresent::new_err(e.to_string()))?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Sets the person that requested the song
    fn requester(mut slf: PyRefMut<Self>, requester: u64) -> PyRefMut<Self> {
        slf.builder.requester = Some(requester.into());
        slf
    }

    /// Sets if the current playing track should be replaced with this new one.
    fn replace(mut slf: PyRefMut<Self>, replace: bool) -> PyRefMut<Self> {
        slf.builder.replace = replace;
        slf
    }

    /// Sets the time the track will start at in seconds.
    fn start_time_secs(mut slf: PyRefMut<Self>, start: u64) -> PyRefMut<Self> {
        slf.builder.start = Duration::from_secs(start).as_millis() as u64;
        slf
    }

    /// Sets the time the track will finish at in seconds.
    fn finish_time_secs(mut slf: PyRefMut<Self>, finish: u64) -> PyRefMut<Self> {
        slf.builder.finish = Duration::from_secs(finish).as_millis() as u64;
        slf
    }

    /// Sets the time the track will start at in milliseconds.
    fn start_time_millis(mut slf: PyRefMut<Self>, start: u64) -> PyRefMut<Self> {
        slf.builder.start = start;
        slf
    }

    /// Sets the time the track will finish at in milliseconds.
    fn finish_time_millis(mut slf: PyRefMut<Self>, finish: u64) -> PyRefMut<Self> {
        slf.builder.finish = finish;
        slf
    }
}
