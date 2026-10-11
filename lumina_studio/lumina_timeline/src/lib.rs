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

#[derive(Clone, Debug)]
pub struct VideoTrack {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
    pub locked: bool,
    pub clips: Vec<TimelineClip>,
}

#[derive(Clone, Debug)]
pub struct AudioTrack {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineClip {
    pub id: Uuid,
    pub asset_id: Uuid,

    // Position on the timeline
    pub timeline_in: RationalTime,
    pub timeline_out: RationalTime,

    // Position bounding the source media
    pub source_in: RationalTime,
    pub source_out: RationalTime,

    pub speed_multiplier: f64,
}

impl TimelineClip {
    pub fn duration(&self) -> RationalTime {
        RationalTime::new(
            self.timeline_out.value - self.timeline_in.value,
            self.timeline_in.timescale
        )
    }

    /// Ripple Edit: Changes the in or out point, shifting all subsequent clips forward or backward.
    pub fn trim_ripple(&mut self, delta: i64, from_head: bool) {
        if from_head {
            self.timeline_in.value += delta;
            self.source_in.value += delta;
        } else {
            self.timeline_out.value += delta;
            self.source_out.value += delta;
        }
        // Note: Timeline wrapper must handle shifting the subsequent clips in the track
    }

    /// Roll Edit: Adjusts the out point of this clip and the in point of the next clip simultaneously
    /// (Handled at the Track level)

    /// Slip Edit: Changes the source in/out points without changing timeline position or duration.
    pub fn trim_slip(&mut self, delta: i64) {
        self.source_in.value += delta;
        self.source_out.value += delta;
    }

    /// Slide Edit: Moves the clip left or right on the timeline, adjusting the neighboring clips' boundaries.
    pub fn trim_slide(&mut self, delta: i64) {
        self.timeline_in.value += delta;
        self.timeline_out.value += delta;
        // Note: Timeline wrapper must handle trimming the adjacent clips.
    }
}

impl VideoTrack {
    /// Perform a Roll edit between two adjacent clips
    /// Adjusts out-point of clip_a and in-point of clip_b simultaneously.
    pub fn trim_roll(&mut self, clip_idx_a: usize, delta: i64) -> Result<(), String> {
        let clip_idx_b = clip_idx_a + 1;
        if clip_idx_b >= self.clips.len() {
            return Err("No adjacent clip found for roll edit.".into());
        }

        // Adjust out point of A
        self.clips[clip_idx_a].timeline_out.value += delta;
        self.clips[clip_idx_a].source_out.value += delta;

        // Adjust in point of B
        self.clips[clip_idx_b].timeline_in.value += delta;
        self.clips[clip_idx_b].source_in.value += delta;

        Ok(())
    }
}
