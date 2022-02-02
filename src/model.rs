use lavalink_rs::model::{
    Band as LavaBand, Info as LavaInfo, Node as LavaNode, PlayerDestroyed as LavaPlayerDestroyed,
    PlayerUpdate as LavaPlayerUpdate, PlaylistInfo as LavaPlaylistInfo, Stats as LavaStats,
    Track as LavaTrack, TrackException as LavaTrackException, TrackFinish as LavaTrackFinish,
    TrackQueue as LavaTrackQueue, TrackStart as LavaTrackStart, TrackStuck as LavaTrackStuck,
    Tracks as LavaTracks, WebSocketClosed as LavaWebSocketClosed,
};
use lavalink_rs::typemap_rev::TypeMapKey;
use pyo3::{prelude::*, types::PyDict};

struct NodeData;

impl TypeMapKey for NodeData {
    type Value = PyObject;
}

/// This is never actually used, a dictionary is used instead. If you use a 3rd party method of
/// joining a voice channel, you can get this values from the `VOICE_STATE_UPDATE` and
/// `VOICE_SERVER_UPDATE` events, and use raw_handle_event_voice_state_update() +
/// raw_handle_event_voice_server_update() or manually build a dict with them.
///
/// With hikari:
/// ```py
/// @bot.listen()
/// async def voice_state_update(event: hikari.VoiceStateUpdateEvent) -> None:
///     await bot.data.lavalink.raw_handle_event_voice_state_update(
///         event.state.guild_id,
///         event.state.user_id,
///         event.state.session_id,
///         event.state.channel_id,
///     )
///
/// @bot.listen()
/// async def voice_server_update(event: hikari.VoiceServerUpdateEvent) -> None:
///     await bot.data.lavalink.raw_handle_event_voice_server_update(
///         event.guild_id, event.endpoint, event.token
///     )
/// ```
///
/// Fields:
///
/// - `guild_id` : `Unsigned 64 bit integer`
/// - `channel_id` : `Unsigned 64 bit integer`
/// - `endpoint` : `String`
/// - `token` : `String`
/// - `session_id` : `String`
#[pyclass]
pub struct ConnectionInfo;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Tracks {
    pub inner: LavaTracks,
}

#[pymethods]
impl Tracks {
    #[getter]
    /// Contains `String`
    fn load_type(&self) -> String {
        self.inner.load_type.clone()
    }

    #[getter]
    /// Information about the playlist.
    ///
    /// Contains `Option<PlaylistInfo>`
    fn playlist_info(&self) -> Option<PlaylistInfo> {
        self.inner
            .playlist_info
            .as_ref()
            .map(|pi| PlaylistInfo { inner: pi.clone() })
    }

    #[getter]
    /// The tracks that can be played
    ///
    /// Contains `List<Track>`
    fn tracks(&self) -> Vec<Track> {
        self.inner
            .tracks
            .iter()
            .map(|i| Track { inner: i.clone() })
            .collect()
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Track {
    pub inner: LavaTrack,
}

#[pymethods]
impl Track {
    #[getter]
    /// The playable track.
    ///
    /// Contains `String`
    fn get_track(&self) -> String {
        self.inner.track.clone()
    }

    #[getter]
    /// Information about the track.
    ///
    /// Contains `Option<Info>`
    fn get_info(&self) -> Option<Info> {
        self.inner.info.as_ref().map(|i| Info { inner: i.clone() })
    }

    #[setter]
    fn set_track(&mut self, track: String) {
        self.inner.track = track;
    }

    #[setter]
    fn set_info(&mut self, info: Option<Info>) {
        self.inner.info = info.map(|i| i.inner);
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct TrackQueue {
    pub inner: LavaTrackQueue,
}

#[pymethods]
impl TrackQueue {
    #[getter]
    /// The playable track.
    ///
    /// Contains `Track`
    fn track(&self) -> Track {
        Track {
            inner: self.inner.track.clone(),
        }
    }

    #[getter]
    /// The time the track will start at.
    ///
    /// Contains `Unsigned 64 bit integer`
    fn start_time(&self) -> u64 {
        self.inner.start_time
    }

    #[getter]
    /// The time the track will finish at.
    ///
    /// Contains `Option<Unsigned 64 bit integer>`
    fn end_time(&self) -> Option<u64> {
        self.inner.end_time
    }

    #[getter]
    /// The user id who requested the track if set by the `PlayBuilder`
    ///
    /// Contains `Option<Unsigned 64 bit integer>`
    fn requester(&self) -> Option<u64> {
        self.inner.requester.map(|u| u.0)
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PlaylistInfo {
    pub inner: LavaPlaylistInfo,
}

#[pymethods]
impl PlaylistInfo {
    #[getter]
    /// Contains `Option<Signed 64 bit integer>`
    fn selected_track(&self) -> Option<i64> {
        self.inner.selected_track
    }

    #[getter]
    /// The name of the playlist.
    ///
    /// Contains `Option<String>`
    fn name(&self) -> Option<String> {
        self.inner.name.as_ref().cloned()
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Info {
    pub inner: LavaInfo,
}

#[pymethods]
impl Info {
    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn get_length(&self) -> u64 {
        self.inner.length
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn get_position(&self) -> u64 {
        self.inner.position
    }

    #[getter]
    /// Contains `bool`
    fn get_is_seekable(&self) -> bool {
        self.inner.is_seekable
    }

    #[getter]
    /// Contains `bool`
    fn get_is_stream(&self) -> bool {
        self.inner.is_stream
    }

    #[getter]
    /// Contains `String`
    fn get_identifier(&self) -> String {
        self.inner.identifier.clone()
    }

    #[getter]
    /// Contains `String`
    fn get_author(&self) -> String {
        self.inner.author.clone()
    }

    #[getter]
    /// Contains `String`
    fn get_title(&self) -> String {
        self.inner.title.clone()
    }

    #[getter]
    /// Contains `String`
    fn get_uri(&self) -> String {
        self.inner.uri.clone()
    }

    #[setter]
    fn set_length(&mut self, length: u64) {
        self.inner.length = length
    }

    #[setter]
    fn set_position(&mut self, position: u64) {
        self.inner.position = position
    }

    #[setter]
    fn set_is_seekable(&mut self, is_seekable: bool) {
        self.inner.is_seekable = is_seekable
    }

    #[setter]
    fn set_is_stream(&mut self, is_stream: bool) {
        self.inner.is_stream = is_stream
    }

    #[setter]
    fn set_identifier(&mut self, identifier: String) {
        self.inner.identifier = identifier
    }

    #[setter]
    fn set_author(&mut self, author: String) {
        self.inner.author = author
    }

    #[setter]
    fn set_title(&mut self, title: String) {
        self.inner.title = title
    }

    #[setter]
    fn set_uri(&mut self, uri: String) {
        self.inner.uri = uri
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Node {
    pub inner: LavaNode,
}

#[pymethods]
impl Node {
    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn get_guild(&self) -> u64 {
        self.inner.guild.0
    }

    #[getter]
    /// Contains `Unsigned 16 bit integer`
    fn get_volume(&self) -> u16 {
        self.inner.volume
    }

    #[getter]
    /// Contains `bool`
    fn get_is_paused(&self) -> bool {
        self.inner.is_paused
    }

    #[getter]
    /// Contains `bool`
    fn get_is_on_loops(&self) -> bool {
        self.inner.is_on_loops
    }

    #[getter]
    /// Contains `Option<TrackQueue>`
    fn get_now_playing(&self) -> Option<TrackQueue> {
        self.inner
            .now_playing
            .as_ref()
            .map(|i| TrackQueue { inner: i.clone() })
    }

    #[getter]
    /// Contains `List<TrackQueue>`
    fn get_queue(&self) -> Vec<TrackQueue> {
        self.inner
            .queue
            .iter()
            .map(|i| TrackQueue { inner: i.clone() })
            .collect()
    }

    #[setter]
    fn set_guild(&mut self, guild_id: u64) {
        self.inner.guild.0 = guild_id;
    }

    #[setter]
    fn set_volume(&mut self, volume: u16) {
        self.inner.volume = volume
    }

    #[setter]
    fn set_is_paused(&mut self, is_paused: bool) {
        self.inner.is_paused = is_paused;
    }

    #[setter]
    fn set_is_on_loops(&mut self, is_on_loops: bool) {
        self.inner.is_on_loops = is_on_loops;
    }

    #[setter]
    fn set_now_playing(&mut self, track: Option<TrackQueue>) {
        self.inner.now_playing = track.map(|t| t.inner);
    }

    #[setter]
    fn set_queue(&mut self, queue: Vec<TrackQueue>) {
        self.inner.queue = queue.iter().map(|i| i.inner.clone()).collect();
    }

    /// Use this to get the currently stored data on the Node.
    ///
    /// `T` is whatever type you give to `set_data`'s data parameter, but if you call this method before it,
    /// it will default to a Dict.
    ///
    /// Returns `T`
    #[pyo3(text_signature = "($self, /)")]
    fn get_data<'a>(&self, py: Python<'a>) -> Py<PyAny> {
        let data_lock = self.inner.data.clone();
        let dict = PyDict::new(py).into_py(py);

        let contains_key = data_lock.read().contains_key::<NodeData>();

        if !contains_key {
            data_lock.write().insert::<NodeData>(dict)
        }

        let data_read = data_lock.read();
        data_read.get::<NodeData>().unwrap().clone()
    }

    /// Use this to set the tored data of the Node.
    ///
    /// Returns `None`
    #[pyo3(text_signature = "($self, data, /)")]
    fn set_data(&self, _py: Python, data: PyObject) {
        let data_lock = self.inner.data.clone();

        data_lock.write().insert::<NodeData>(data);
    }
}

#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "($self, guild_id, /)")]
/// See `Lavalink.equalize_all` for more info.
///
/// ```py
/// band_num: int = 14 # 0 to 14
/// bain: float = 0.125 # -0.25 to 1.0
///
/// band = Band(band_num, gain)
/// ```
pub struct Band {
    pub inner: LavaBand,
}

#[pymethods]
impl Band {
    #[new]
    fn new(band: u8, gain: f64) -> Self {
        Self {
            inner: LavaBand { band, gain },
        }
    }

    #[getter]
    /// Contains `Unsigned 8 bit integer`
    fn get_band(&self) -> u8 {
        self.inner.band
    }

    #[getter]
    /// Contains `64 bit float`
    fn get_gain(&self) -> f64 {
        self.inner.gain
    }

    #[setter]
    fn set_band(&mut self, val: u8) {
        self.inner.band = val
    }

    #[setter]
    fn set_gain(&mut self, val: f64) {
        self.inner.gain = val
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Stats {
    pub inner: LavaStats,
}

#[pymethods]
impl Stats {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn playing_players(&self) -> i64 {
        self.inner.playing_players
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn players(&self) -> i64 {
        self.inner.players
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn uptime(&self) -> i64 {
        self.inner.uptime
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn memory_reservable(&self) -> i64 {
        self.inner.memory.reservable
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn memory_used(&self) -> i64 {
        self.inner.memory.used
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn memory_free(&self) -> i64 {
        self.inner.memory.free
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn memory_allocated(&self) -> i64 {
        self.inner.memory.allocated
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn cpu_cores(&self) -> i64 {
        self.inner.cpu.cores
    }

    #[getter]
    /// Contains `64 bit floating point`
    fn cpu_system_load(&self) -> f64 {
        self.inner.cpu.system_load
    }

    #[getter]
    /// Contains `64 bit floating point`
    fn cpu_lavalink_load(&self) -> f64 {
        self.inner.cpu.lavalink_load
    }

    #[getter]
    /// Contains `Optional Signed 64 bit integer`
    fn frame_stats_sent(&self) -> Option<i64> {
        self.inner.frame_stats.as_ref().map(|i| i.sent)
    }

    #[getter]
    /// Contains `Optional Signed 64 bit integer`
    fn frame_stats_deficit(&self) -> Option<i64> {
        self.inner.frame_stats.as_ref().map(|i| i.deficit)
    }

    #[getter]
    /// Contains `Optional Signed 64 bit integer`
    fn frame_stats_nulled(&self) -> Option<i64> {
        self.inner.frame_stats.as_ref().map(|i| i.nulled)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PlayerUpdate {
    pub inner: LavaPlayerUpdate,
}

#[pymethods]
impl PlayerUpdate {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn guild_id(&self) -> u64 {
        self.inner.guild_id.0
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn state_position(&self) -> i64 {
        self.inner.state.position
    }

    #[getter]
    /// Contains `Signed 64 bit integer`
    fn state_time(&self) -> i64 {
        self.inner.state.time
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TrackStart {
    pub inner: LavaTrackStart,
}

#[pymethods]
impl TrackStart {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn guild_id(&self) -> u64 {
        self.inner.guild_id.0
    }

    #[getter]
    /// Contains `String`
    fn track_start_type(&self) -> String {
        self.inner.track_start_type.clone()
    }

    #[getter]
    /// Contains `String`
    fn track(&self) -> String {
        self.inner.track.clone()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TrackFinish {
    pub inner: LavaTrackFinish,
}

#[pymethods]
impl TrackFinish {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn guild_id(&self) -> u64 {
        self.inner.guild_id.0
    }

    #[getter]
    /// Contains `String`
    fn track_finish_type(&self) -> String {
        self.inner.track_finish_type.clone()
    }

    #[getter]
    /// Contains `String`
    fn track(&self) -> String {
        self.inner.track.clone()
    }

    #[getter]
    /// Contains `String`
    fn reason(&self) -> String {
        self.inner.reason.clone()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TrackException {
    pub inner: LavaTrackException,
}

#[pymethods]
impl TrackException {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn guild_id(&self) -> u64 {
        self.inner.guild_id.0
    }

    #[getter]
    /// Contains `String`
    fn track_exception_type(&self) -> String {
        self.inner.track_exception_type.clone()
    }

    #[getter]
    /// Contains `String`
    fn track(&self) -> String {
        self.inner.track.clone()
    }

    #[getter]
    /// Contains `String`
    fn error(&self) -> String {
        self.inner.track.clone()
    }

    #[getter]
    /// Contains `String`
    fn exception_severity(&self) -> String {
        self.inner.exception.severity.clone()
    }

    #[getter]
    /// Contains `String`
    fn exception_cause(&self) -> String {
        self.inner.exception.cause.clone()
    }

    #[getter]
    /// Contains `String`
    fn exception_message(&self) -> String {
        self.inner.exception.message.clone()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TrackStuck {
    pub inner: LavaTrackStuck,
}

#[pymethods]
impl TrackStuck {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn guild_id(&self) -> u64 {
        self.inner.guild_id.0
    }

    #[getter]
    /// Contains `String`
    fn track_stuck_type(&self) -> String {
        self.inner.track_stuck_type.clone()
    }

    #[getter]
    /// Contains `String`
    fn track(&self) -> String {
        self.inner.track.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn threshold_ms(&self) -> u64 {
        self.inner.threshold_ms
    }
}

#[pyclass]
#[derive(Clone)]
pub struct WebSocketClosed {
    pub inner: LavaWebSocketClosed,
}

#[pymethods]
impl WebSocketClosed {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn guild_id(&self) -> u64 {
        self.inner.guild_id.0
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn user_id(&self) -> u64 {
        self.inner.user_id.0
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn code(&self) -> u64 {
        self.inner.code
    }

    #[getter]
    /// Contains `String`
    fn websocket_closed_type(&self) -> String {
        self.inner.websocket_closed_type.clone()
    }

    #[getter]
    /// Contains `bool`
    fn by_remote(&self) -> bool {
        self.inner.by_remote
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PlayerDestroyed {
    pub inner: LavaPlayerDestroyed,
}

#[pymethods]
impl PlayerDestroyed {
    #[getter]
    /// Contains `String`
    fn op(&self) -> String {
        self.inner.op.clone()
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn guild_id(&self) -> u64 {
        self.inner.guild_id.0
    }

    #[getter]
    /// Contains `Unsigned 64 bit integer`
    fn user_id(&self) -> u64 {
        self.inner.user_id.0
    }

    #[getter]
    /// Contains `String`
    fn player_destroyed_type(&self) -> String {
        self.inner.player_destroyed_type.clone()
    }

    #[getter]
    /// Contains `bool`
    fn cleanup(&self) -> bool {
        self.inner.cleanup
    }
}
