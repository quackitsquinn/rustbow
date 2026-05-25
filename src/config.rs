//! Configuration for Rustbow.
use clap::ValueEnum;
use std::borrow::Cow;

/// Configuration for Rustbow.
#[derive(Debug, Clone)]
pub struct RustBowConfig {
    /// A string of characters to use instead of random characters. Default is "@#$%&?".
    pub charset: Charset,
    /// The foreground color config.
    pub foreground: ColorConfig,
    /// The background color config. If `None`, no background color will be used.
    pub background: Option<ColorConfig>,
    /// The speed of the animation, in ms per character.
    pub speed_ms: f32,
}

/// The configuration for a cycling color in Rustbow.
#[derive(Debug, Clone, Copy)]
pub struct ColorConfig {
    /// The amount to increment the hue by.
    pub change_rate: f32,
    /// The initial hue of the outputted colors, between 0 and 360.
    pub initial_hue: f32,
    /// The saturation of the outputted colors, between 0 and 1.
    pub saturation: f32,
    /// The value of the outputted colors, between 0 and 1.
    pub lightness: f32,
}

impl ColorConfig {
    /// Modifies the config with the given modifier. If a field in the modifier is `None`, the original value is used.
    pub fn modify_with(&self, modifier: &ColorConfigModifier) -> Self {
        Self {
            change_rate: modifier.change_rate.unwrap_or(self.change_rate),
            initial_hue: modifier.initial_hue.unwrap_or(self.initial_hue),
            saturation: modifier.saturation.unwrap_or(self.saturation),
            lightness: modifier.lightness.unwrap_or(self.lightness),
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            change_rate: 0.001,
            initial_hue: 0.0,
            saturation: 1.0,
            lightness: 0.8,
        }
    }
}

impl RustBowConfig {
    /// Modifies the config with the given modifier. If a field in the modifier is `None`, the original value is used.
    pub fn modify_with(&self, modifier: &RustBowConfigModifier) -> Self {
        Self {
            charset: modifier
                .charset
                .clone()
                .unwrap_or_else(|| self.charset.clone()),
            foreground: self
                .foreground
                .modify_with(&modifier.foreground_config.unwrap_or_default()),
            background: modifier
                .background_config
                .map(|bg_mod| self.background.unwrap_or_default().modify_with(&bg_mod)),
            speed_ms: modifier.speed_ms.unwrap_or(1.0),
        }
    }
}

impl Default for RustBowConfig {
    fn default() -> Self {
        Self {
            charset: CharsetTemplate::Default.to_charset(),
            foreground: ColorConfig::default(),
            background: None,
            speed_ms: 0.0,
        }
    }
}

/// A modifier for the RustBowConfig. This is used to modify the config with command line arguments.
#[derive(Debug, Clone)]
pub struct RustBowConfigModifier {
    /// The template to use instead of random characters. use <TODO> to list the templates
    pub charset: Option<Charset>,
    /// The foreground color config modifier.
    pub foreground_config: Option<ColorConfigModifier>,
    /// The background color config modifier.
    pub background_config: Option<ColorConfigModifier>,
    /// The speed of the animation, in ms per character.
    pub speed_ms: Option<f32>,
}

/// A modifier for the ColorConfig. This is used to modify the color config with command line arguments.
#[derive(Debug, Clone, Copy, Default)]
pub struct ColorConfigModifier {
    /// The amount to increment the hue by.
    pub change_rate: Option<f32>,
    /// The initial hue of the outputted colors, between 0 and 360.
    pub initial_hue: Option<f32>,
    /// The saturation of the outputted colors, between 0 and 1.
    pub saturation: Option<f32>,
    /// The value of the outputted colors, between 0 and 1.
    pub lightness: Option<f32>,
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
    pub const fn to_charset(&self) -> Charset {
        match self {
            Self::Default => Charset::borrowed(Self::DEFAULT),
            Self::Blocks => Charset::borrowed(Self::BLOCKS),
            Self::Corners => Charset::borrowed(Self::CORNERS),
            Self::CornerBlock => Charset::borrowed(Self::CORNER_BLOCK),
        }
    }
}
