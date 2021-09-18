use lavalink_rs::model::{
    Info as LavaInfo, Node as LavaNode, PlaylistInfo as LavaPlaylistInfo, Track as LavaTrack,
    TrackQueue as LavaTrackQueue, Tracks as LavaTracks, Band as LavaBand,
};
use pyo3::prelude::*;

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
    /// Returns `String`
    fn load_type(&self) -> String {
        self.inner.load_type.clone()
    }

    #[getter]
    /// Information about the playlist.
    ///
    /// Returns `Option<PlaylistInfo>`
    fn playlist_info(&self) -> Option<PlaylistInfo> {
        self.inner.playlist_info.as_ref().map(|pi| PlaylistInfo { inner: pi.clone() })
    }

    #[getter]
    /// The tracks that can be played
    ///
    /// Returns `List<Track>`
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
    /// Returns `String`
    fn track(&self) -> String {
        self.inner.track.clone()
    }

    #[getter]
    /// Information about the track.
    ///
    /// Returns `Option<Info>`
    fn info(&self) -> Option<Info> {
        self.inner.info.as_ref().map(|i| Info { inner: i.clone() })
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
    /// Returns `Track`
    fn track(&self) -> Track {
        Track {
            inner: self.inner.track.clone(),
        }
    }

    #[getter]
    /// The time the track will start at.
    ///
    /// Returns `Unsigned 64 bit integer`
    fn start_time(&self) -> u64 {
        self.inner.start_time
    }

    #[getter]
    /// The time the track will finish at.
    ///
    /// Returns `Option<Unsigned 64 bit integer>`
    fn end_time(&self) -> Option<u64> {
        self.inner.end_time
    }

    #[getter]
    /// The user id who requested the track if set by the `PlayBuilder`
    ///
    /// Returns `Option<Unsigned 64 bit integer>`
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
    /// Returns `Option<Signed 64 bit integer>`
    fn selected_track(&self) -> Option<i64> {
        self.inner.selected_track
    }

    #[getter]
    /// The name of the playlist.
    ///
    /// Returns `Option<String>`
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
    /// Returns `Unsigned 64 bit integer`
    fn length(&self) -> u64 {
        self.inner.length
    }

    #[getter]
    /// Returns `Unsigned 64 bit integer`
    fn position(&self) -> u64 {
        self.inner.position
    }

    #[getter]
    /// Returns `bool`
    fn is_seekable(&self) -> bool {
        self.inner.is_seekable
    }

    #[getter]
    /// Returns `bool`
    fn is_stream(&self) -> bool {
        self.inner.is_stream
    }

    #[getter]
    /// Returns `String`
    fn identifier(&self) -> String {
        self.inner.identifier.clone()
    }

    #[getter]
    /// Returns `String`
    fn author(&self) -> String {
        self.inner.author.clone()
    }

    #[getter]
    /// Returns `String`
    fn title(&self) -> String {
        self.inner.title.clone()
    }

    #[getter]
    /// Returns `String`
    fn uri(&self) -> String {
        self.inner.uri.clone()
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
    /// Returns `Unsigned 64 bit integer`
    fn guild(&self) -> u64 {
        self.inner.guild.0
    }

    #[getter]
    /// Returns `Unsigned 16 bit integer`
    fn volume(&self) -> u16 {
        self.inner.volume
    }

    #[getter]
    /// Returns `bool`
    fn is_paused(&self) -> bool {
        self.inner.is_paused
    }

    #[getter]
    /// Returns `bool`
    fn is_on_loops(&self) -> bool {
        self.inner.is_on_loops
    }

    #[getter]
    /// Returns `Option<TrackQueue>`
    fn now_playing(&self) -> Option<TrackQueue> {
        self.inner
            .now_playing
            .as_ref()
            .map(|i| TrackQueue { inner: i.clone() })
    }

    #[getter]
    /// Returns `List<TrackQueue>`
    fn queue(&self) -> Vec<TrackQueue> {
        self.inner
            .queue
            .iter()
            .map(|i| TrackQueue { inner: i.clone() })
            .collect()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Band {
    pub inner: LavaBand,
}


#[pymethods]
impl Band {
    #[new]
    fn new(band: u8, gain: f64) -> Self {
        Self { inner: LavaBand { band, gain } }
    }

    #[getter]
    /// Returns `Unsigned 8 bit integer`
    fn get_band(&self) -> u8 {
        self.inner.band
    }

    #[getter]
    /// Returns `64 bit float`
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
