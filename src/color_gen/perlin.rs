use color::Rgba8;
use noise::NoiseFn;
use rand::rngs::ThreadRng;

use crate::{
    color_gen::{ColorGenerator, MixMethod},
    config::ColorGeneratorConfig,
};

/// A color generator that uses Perlin noise to generate colors based on the position of the character.
pub struct PerlinModifier {
    inner: Box<dyn ColorGenerator>,
    noise: noise::Perlin,
    scale: f64,
    mix_method: MixMethod,
}

impl PerlinModifier {
    /// Creates a new `PerlinModifier` with the given inner generator, scale, and seed for the noise function.
    pub fn new(
        inner: Box<dyn ColorGenerator>,
        scale: f64,
        mix_method: MixMethod,
        seed: u32,
    ) -> Self {
        Self {
            inner,
            noise: noise::Perlin::new(seed),
            mix_method,
            scale,
        }
    }
}

impl ColorGenerator for PerlinModifier {
    fn next_frame(&mut self, rand: &mut ThreadRng) {
        self.inner.next_frame(rand);
    }

    fn get(&mut self, rand: &mut ThreadRng, pos: (u16, u16)) -> Rgba8 {
        let base_color = self.inner.get(rand, pos);
        let noise_value = self
            .noise
            .get([pos.0 as f64 * self.scale, pos.1 as f64 * self.scale]);
        let modifier_color = Rgba8 {
            r: ((noise_value + 1.0) / 2.0 * 255.0) as u8,
            g: ((noise_value + 1.0) / 2.0 * 255.0) as u8,
            b: ((noise_value + 1.0) / 2.0 * 255.0) as u8,
            a: 255,
        };
        self.mix_method.mix(base_color, modifier_color)
    }
}

/// A config for the `PerlinModifier`.
#[derive(Debug, Clone)]
pub struct PerlinConfig {
    /// The scale of the noise function. Higher values will result in more rapid changes in color, while lower values will result in smoother transitions.
    pub scale: f64,
    /// The method for mixing the noise-generated color with the base color.
    pub mix_method: MixMethod,
    /// The seed for the noise function, which will determine the specific pattern of colors generated.
    pub seed: u32,
    /// The inner color generator that will generate the base colors before the noise modifier is applied.
    pub source: Box<ColorGeneratorConfig>, // Box because ColorGeneratorConfig will have a PerlinConfig variant
}
