//! A module for generating colors for the RustBow animation.
use color::{Hsl, OpaqueColor, Rgba8};
use rand::rngs::ThreadRng;

/// A trait for generating colors for the RustBow animation.
pub trait ColorGenerator {
    /// Generates the next frame of the animation. This should update any internal state necessary for generating the next colors.
    fn next_frame(&mut self);

    /// Generates the next color for the given position.
    fn get(&mut self, rand: &mut ThreadRng, pos: (u16, u16)) -> Rgba8;
}

/// A color generator that shifts the hue of the foreground and background colors by a certain rate each frame.
pub struct HueShiftGenerator {
    rate: f32,
    color: OpaqueColor<Hsl>,
}

impl HueShiftGenerator {
    /// Creates a new `HueShiftGenerator` with the given rate and initial foreground color.
    pub fn new(rate: f32, initial_fg_color: OpaqueColor<Hsl>) -> Self {
        Self {
            rate,
            color: initial_fg_color,
        }
    }
}

impl ColorGenerator for HueShiftGenerator {
    fn next_frame(&mut self) {}

    fn get(&mut self, _rand: &mut ThreadRng, _pos: (u16, u16)) -> Rgba8 {
        self.color = self.color.map_hue(|h| (h + self.rate) % 360.);
        self.color.to_rgba8()
    }
}
