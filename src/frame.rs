use std::time::{Duration, Instant};

pub struct FrameTracker {
    now: Instant,
    next: Instant,
    frame_duration: Duration,
}

impl FrameTracker {
    /// Creates a new `FrameTracker` with the given frames per second. This will track the time between frames and allow for sleeping until the next frame is ready.
    pub fn new(frames_per_second: f32) -> Self {
        let frame_duration = if frames_per_second > 0. {
            Duration::from_secs_f32(1. / frames_per_second)
        } else {
            Duration::ZERO
        };
        let now = Instant::now();
        Self {
            now,
            next: now + frame_duration,
            frame_duration,
        }
    }

    /// Ends the current frame, sleeping until the next frame is ready if necessary and updating the current time and next frame time.
    pub fn end_frame(&mut self) {
        let now = Instant::now();
        if self.next > now {
            std::thread::sleep(self.next - now);
        }
        self.now = now;
        self.next = self.now + self.frame_duration;
    }
}
