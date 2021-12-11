use crate::error;
use crate::model;
use crate::Lavalink;

use pyo3::prelude::*;

use lavalink_rs::{
    async_trait, gateway::LavalinkEventHandler as LavalinkEventHandlerTrait, model::*,
    LavalinkClient,
};

#[pyclass]
#[derive(Clone)]
/// The lavalink event handler. This is a trait, so it defines the structure a class should have.
///
/// Make a class with the methods and signatures this class defines, and add that class to
/// `LavalinkBuilder.build()`
///
/// If code inside any of the event raises an error, the traceback will be printed to stderr, and
/// the variables `sys.last_type`, `sys.last_value` and `sys.last_traceback` will be set to the type, value
/// and traceback of the printed exception respectively.
///
/// Some examples:
///
/// ```py
/// # Just a single event
/// class EventHandler:
///     async def track_start(self, lava_client, event):
///         print(event)
///
/// lavalink_client = await client_builder.build(EventHandler)
/// ```
///
/// ```py
/// # No events
/// class EventHandler:
///     pass
///
/// lavalink_client = await client_builder.build(EventHandler)
/// ```
///
/// ```py
/// # Just a single event
/// class EventHandler:
///     async def stats(self, lava_client, event):
///         print(event)
///     async def player_update(self, lava_client, event):
///         print(event)
///     async def track_start(self, lava_client, event):
///         print(event)
///     async def track_finish(self, lava_client, event):
///         print(event)
///     async def track_exception(self, lava_client, event):
///         print(event)
///     async def track_stuck(self, lava_client, event):
///         print(event)
///     async def websocket_closed(self, lava_client, event):
///         print(event)
///     async def player_destroyed(self, lava_client, event):
///         print(event)
///
/// lavalink_client = await client_builder.build(EventHandler)
/// ```
pub struct LavalinkEventHandler {
    pub inner: PyObject,
    pub current_loop: PyObject,
}

#[async_trait]
impl LavalinkEventHandlerTrait for LavalinkEventHandler {
    async fn stats(&self, client: LavalinkClient, event: Stats) {
        let event = model::Stats { inner: event };
        call_event(self, client, event, "stats");
    }
    async fn player_update(&self, client: LavalinkClient, event: PlayerUpdate) {
        let event = model::PlayerUpdate { inner: event };
        call_event(self, client, event, "player_update");
    }
    async fn track_start(&self, client: LavalinkClient, event: TrackStart) {
        let event = model::TrackStart { inner: event };
        call_event(self, client, event, "track_start");
    }
    async fn track_finish(&self, client: LavalinkClient, event: TrackFinish) {
        let event = model::TrackFinish { inner: event };
        call_event(self, client, event, "track_finish");
    }
    async fn track_exception(&self, client: LavalinkClient, event: TrackException) {
        let event = model::TrackException { inner: event };
        call_event(self, client, event, "track_exception");
    }
    async fn track_stuck(&self, client: LavalinkClient, event: TrackStuck) {
        let event = model::TrackStuck { inner: event };
        call_event(self, client, event, "track_stuck");
    }
    async fn websocket_closed(&self, client: LavalinkClient, event: WebSocketClosed) {
        let event = model::WebSocketClosed { inner: event };
        call_event(self, client, event, "websocket_closed");
    }
    async fn player_destroyed(&self, client: LavalinkClient, event: PlayerDestroyed) {
        let event = model::PlayerDestroyed { inner: event };
        call_event(self, client, event, "player_destroyed");
    }
}

#[pymethods]
impl LavalinkEventHandler {
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Periodic event that returns the statistics of the server.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `Stats`
    ///
    /// Returns: `Future<None>`
    fn stats(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a player updates.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `PlayerUpdate`
    ///
    /// Returns: `Future<None>`
    fn player_update(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track starts playing.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackStart`
    ///
    /// Returns: `Future<None>`
    fn track_start(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track finishes playing.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackFinish`
    ///
    /// Returns: `Future<None>`
    fn track_finish(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track raises an exception on the Lavalink server.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackException`
    ///
    /// Returns: `Future<None>`
    fn track_exception(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track gets stuck while playing.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackStuck`
    ///
    /// Returns: `Future<None>`
    fn track_stuck(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when the websocket connection to the voice channel closes.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `WebSocketClosed`
    ///
    /// Returns: `Future<None>`
    fn websocket_closed(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when the player gets destroyed on a guild.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `PlayerDestroyed`
    ///
    /// Returns: `Future<None>`
    fn player_destroyed(&self) {}
}

fn call_event<T: Send + Sync + pyo3::IntoPy<PyObject> + 'static>(
    handler: &LavalinkEventHandler,
    client: LavalinkClient,
    event: T,
    name: &'static str,
) {
    let slf1 = handler.clone();
    let slf2 = handler.clone();

    Python::with_gil(|py| {
        let current_loop = slf1.current_loop.as_ref(py);

        pyo3_asyncio::tokio::future_into_py_with_locals(
            py,
            pyo3_asyncio::TaskLocals::new(current_loop),
            async move {
                let future = Python::with_gil(|py| {
                    let py_event_handler = slf2.inner.as_ref(py);
                    let coro_result = py_event_handler.call_method(
                        name,
                        (Lavalink { lava: client }, event),
                        None,
                    );

                    if let Ok(coro) = coro_result {
                        pyo3_asyncio::tokio::into_future(coro)
                    } else {
                        Err(error::NameError::new_err("Undefined event"))
                    }
                });

                if let Ok(f) = future {
                    if let Err(e) = f.await {
                        Python::with_gil(|py| {
                            e.print_and_set_sys_last_vars(py);
                        });
                    }
                }

                Ok(Python::with_gil(|py| py.None()))
            },
        )
        .unwrap();
    });
}
