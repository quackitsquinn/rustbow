//! A module for generating colors for the RustBow animation.

use color::Rgba8;
use rand::rngs::ThreadRng;

mod hue;
mod perlin;

pub use hue::*;
pub use perlin::*;

/// A trait for generating colors for the RustBow animation.
pub trait ColorGenerator {
    /// Generates the next frame of the animation. This should update any internal state necessary for generating the next colors.
    fn next_frame(&mut self, rand: &mut ThreadRng);

    /// Generates the next color for the given position.
    fn get(&mut self, rand: &mut ThreadRng, pos: (u16, u16)) -> Rgba8;
}

/// A method for mixing two colors together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MixMethod {
    /// Adds the RGB values of the modifier color to the base color, clamping at 255.
    Additive,
    /// Multiplies the RGB values of the base color by the modifier color, treating the modifier color as a value between 0 and 1.
    Multiply,
}

impl MixMethod {
    /// Mixes the base color and modifier color together using the specified mix method.
    pub fn mix(&self, base: Rgba8, modifier: Rgba8) -> Rgba8 {
        match self {
            MixMethod::Additive => Rgba8 {
                r: base.r.saturating_add(modifier.r),
                g: base.g.saturating_add(modifier.g),
                b: base.b.saturating_add(modifier.b),
                a: base.a,
            },
            MixMethod::Multiply => Rgba8 {
                r: ((base.r as u16 * modifier.r as u16) / 255) as u8,
                g: ((base.g as u16 * modifier.g as u16) / 255) as u8,
                b: ((base.b as u16 * modifier.b as u16) / 255) as u8,
                a: base.a,
            },
        }
    }
}
