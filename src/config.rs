//! Configuration for Rustbow.
use anyhow::Context;
use clap::ValueEnum;
use std::{borrow::Cow, collections::HashMap, str::FromStr, sync::Arc};

use crate::color_gen::{ColorGenerator, HueShiftConfig, HueShiftGenerator};

mod set;
pub use set::{ConfigSet, FromConfig};

/// Configuration for Rustbow.
#[derive(Debug, Clone)]
pub struct RustBowConfig {
    /// A string of characters to use instead of random characters. Default is "@#$%&?".
    pub charset: Charset,
    /// The foreground color config.
    pub foreground: ColorGeneratorConfig,
    /// The background color config. If `None`, no background color will be used.
    pub background: Option<ColorGeneratorConfig>,
    /// The number of frames per second to run the animation at. Default is 0, which means to run as fast as possible.
    pub frames_per_second: f32,
    /// The number of characters to print per frame. Default is 3.
    pub chars_per_frame: usize,
}

impl RustBowConfig {
    /// Modifies the config with the given modifier. If a field in the modifier is `None`, the original value is used.
    pub fn modify_with(&self, modifier: &RustBowConfigModifier) -> Self {
        Self {
            charset: modifier
                .charset
                .clone()
                .unwrap_or_else(|| self.charset.clone()),
            foreground: modifier.foreground_config.unwrap_or(self.foreground),
            background: modifier.background_config.or(self.background),
            frames_per_second: modifier.frames_per_second.unwrap_or(self.frames_per_second),
            chars_per_frame: modifier.chars_per_frame.unwrap_or(self.chars_per_frame),
        }
    }
}

impl Default for RustBowConfig {
    fn default() -> Self {
        Self {
            charset: CharsetTemplate::Default.build_charset(),
            foreground: ColorGeneratorConfig::default(),
            background: None,
            frames_per_second: 0.0,
            chars_per_frame: 3,
        }
    }
}

/// A modifier for the RustBowConfig. This is used to modify the config with command line arguments.
#[derive(Debug, Clone)]
pub struct RustBowConfigModifier {
    /// The template to use instead of random characters. use <TODO> to list the templates
    pub charset: Option<Charset>,
    /// The foreground color config modifier.
    pub foreground_config: Option<ColorGeneratorConfig>,
    /// The background color config modifier.
    pub background_config: Option<ColorGeneratorConfig>,
    /// The number of frames per second to run the animation at. Default is 0, which means to run as fast as possible.
    pub frames_per_second: Option<f32>,
    /// The number of characters to print per frame. Default is 3.
    pub chars_per_frame: Option<usize>,
}

/// A character set.
#[derive(Clone, Debug)]
pub struct Charset {
    /// The chars in the character set.
    pub chars: Cow<'static, [char]>,
}

impl Charset {
    /// Creates a borrowed charset from a static slice of characters.
    pub const fn borrowed(chars: &'static [char]) -> Self {
        Self {
            chars: Cow::Borrowed(chars),
        }
    }

    /// Creates an owned charset from a vector of characters.
    pub fn owned(chars: Vec<char>) -> Self {
        Self {
            chars: Cow::Owned(chars),
        }
    }
}

/// A template for a character set.
#[derive(Debug, Clone, ValueEnum)]
pub enum CharsetTemplate {
    /// The default character set.
    Default,
    /// A character set of blocks.
    Blocks,
    /// A character set of corners.
    Corners,
    /// Corners and blocks, including a white space char.
    CornerBlock,
}

impl CharsetTemplate {
    /// The default character set.
    pub const DEFAULT: &'static [char] = &['@', '#', '$', '%', '&', '?']; // default chars: @#$%&?
    /// A character set of blocks.
    pub const BLOCKS: &'static [char] = &['█', '▒', '░']; // block chars: █▓▒░
    /// A character set of corners.
    pub const CORNERS: &'static [char] = &['▘', '▝', '▖', '▗']; // corner chars: ▘▝▖▗
    /// Corners and blocks, including a white space char.
    pub const CORNER_BLOCK: &'static [char] = &[' ', '█', '▒', '░', '▘', '▝', '▖', '▗']; // corner and block chars: " █▓▒░▘▝▖▗"

    /// Converts the template to a charset.
    pub const fn build_charset(&self) -> Charset {
        match self {
            Self::Default => Charset::borrowed(Self::DEFAULT),
            Self::Blocks => Charset::borrowed(Self::BLOCKS),
            Self::Corners => Charset::borrowed(Self::CORNERS),
            Self::CornerBlock => Charset::borrowed(Self::CORNER_BLOCK),
        }
    }
}

/// A generator for a color generator, using a given config.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ColorGeneratorConfig {
    /// A color generator that shifts the hue of the output color by a defined rate each sample.
    // TODO: allow using per frame instead of per sample
    HueShift(HueShiftConfig),
}

impl ColorGeneratorConfig {
    /// Builds a color generator from the config.
    pub fn build_generator(&self) -> Box<dyn ColorGenerator> {
        match self {
            Self::HueShift(cfg) => Box::new(HueShiftGenerator::new(*cfg)),
        }
    }

    /// Parses a color generator config from a generator type and a string of attributes. The generator type is optional and defaults to "hue_shift"
    pub fn parse(gentype: &str, attribs: &str) -> anyhow::Result<Self> {
        let attributes =
            ConfigSet::from_str(attribs).context("Failed to parse color generator attributes")?;
        match gentype {
            "hue_shift" => Ok(Self::HueShift(HueShiftConfig::from_config(
                &mut attributes.clone(),
            )?)),
            _ => anyhow::bail!("Unknown color generator type: {}", gentype),
        }
    }
}

impl Default for ColorGeneratorConfig {
    fn default() -> Self {
        Self::HueShift(HueShiftConfig::default())
    }
}
