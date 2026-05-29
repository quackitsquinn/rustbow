//! A module for generating colors for the RustBow animation.

use color::{Hsl, OpaqueColor, Rgba8};
use rand::rngs::ThreadRng;

use crate::config::{ConfigSet, FromConfig};

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
    pub fn new(cfg: HueShiftConfig) -> Self {
        Self {
            rate: cfg.rate,
            color: cfg.initial_color,
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

impl Default for HueShiftGenerator {
    fn default() -> Self {
        Self::new(HueShiftConfig::default())
    }
}

/// A config for the `HueShiftGenerator`.
#[derive(Debug, Clone, Copy)]
pub struct HueShiftConfig {
    /// The amount to increment the hue by.
    pub rate: f32,
    /// The initial color of the outputted colors.
    pub initial_color: OpaqueColor<Hsl>,
}

impl FromConfig for HueShiftConfig {
    type Err = anyhow::Error;

    fn from_config(cfg: &mut ConfigSet) -> Result<Self, Self::Err> {
        let rate = cfg
            .take_any(&["r", "cr", "change_rate", "rate", "hue_rate"])?
            .unwrap_or(0.001);
        let initial_hue = cfg
            .take_any(&["ih", "h", "hue", "initial_hue"])?
            .unwrap_or(0.0);

        let sat = cfg.take_any(&["s", "sat", "saturation"])?.unwrap_or(1.0) * 100.;

        let light = cfg.take_any(&["l", "light", "lightness"])?.unwrap_or(0.5) * 100.;

        Ok(Self {
            rate,
            initial_color: OpaqueColor::new([initial_hue, sat, light]),
        })
    }
}

impl Default for HueShiftConfig {
    fn default() -> Self {
        Self {
            rate: 0.001,
            initial_color: OpaqueColor::new([0., 1., 0.5]),
        }
    }
}
