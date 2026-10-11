use lumina_core::RationalTime;
use uuid::Uuid;

// ============================================================================
// Core timeline primitives
// ============================================================================

pub struct Timeline {
    pub id: Uuid,
    pub name: String,
    pub timebase: RationalTime,
    pub video_tracks: Vec<VideoTrack>,
    pub audio_tracks: Vec<AudioTrack>,
}

pub struct VideoTrack {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
    pub locked: bool,
    pub clips: Vec<TimelineClip>,
}

pub struct AudioTrack {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
    pub locked: bool,
    // Add audio clips eventually
}

pub struct TimelineClip {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub timeline_in: RationalTime,
    pub timeline_out: RationalTime,
    pub source_in: RationalTime,
    pub source_out: RationalTime,
    pub speed_multiplier: f64,
}
