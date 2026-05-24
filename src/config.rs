//! Configuration for Rustbow.
use clap::ValueEnum;
use std::{borrow::Cow, str::FromStr, sync::Arc};

use crate::ArcSlice;

/// Configuration for Rustbow.
#[derive(Debug, Clone)]
pub struct RustBowConfig {
    /// A string of characters to use instead of random characters. Default is "@#$%&?".
    pub charset: Charset,
    /// The amount to increment the hue by. Default is 0.001.
    pub change_rate: f32,
    /// The saturation of the outputted colors, between 0 and 1. Default is 1.
    pub saturation: f32,
    /// The value of the outputted colors, between 0 and 1. Default is 1.
    pub lightness: f32,
}

impl RustBowConfig {
    /// Modifies the config with the given modifier. If a field in the modifier is `None`, the original value is used.
    pub fn modify_with(&self, modifier: &RustBowConfigModifier) -> Self {
        Self {
            charset: modifier
                .charset
                .clone()
                .unwrap_or_else(|| self.charset.clone()),
            change_rate: modifier.change_rate.unwrap_or(self.change_rate),
            saturation: modifier.saturation.unwrap_or(self.saturation),
            lightness: modifier.value.unwrap_or(self.lightness),
        }
    }
}

impl Default for RustBowConfig {
    fn default() -> Self {
        Self {
            charset: CharsetTemplate::Default.to_charset(),
            change_rate: 0.001,
            saturation: 1.0,
            lightness: 0.8,
        }
    }
}

/// A modifier for the RustBowConfig. This is used to modify the config with command line arguments.
pub struct RustBowConfigModifier {
    /// The template to use instead of random characters. use <TODO> to list the templates
    pub charset: Option<Charset>,
    /// The amount to increment the hue by. Default is 0.001.
    pub change_rate: Option<f32>,
    /// The saturation of the outputted colors, between 0 and 1. Default is 1.
    pub saturation: Option<f32>,
    /// The value of the outputted colors, between 0 and 1. Default is 1.
    pub value: Option<f32>,
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
}

impl CharsetTemplate {
    /// The default character set.
    pub const DEFAULT: &'static [char] = &['@', '#', '$', '%', '&', '?']; // default chars: @#$%&?
    /// A character set of blocks.
    pub const BLOCKS: &'static [char] = &['█', '▒', '░']; // block chars: █▓▒░
    /// A character set of corners.
    pub const CORNERS: &'static [char] = &['▘', '▝', '▖', '▗']; // corner chars: ▘▝▖▗

    /// Converts the template to a charset.
    pub const fn to_charset(&self) -> Charset {
        match self {
            Self::Default => Charset::borrowed(Self::DEFAULT),
            Self::Blocks => Charset::borrowed(Self::BLOCKS),
            Self::Corners => Charset::borrowed(Self::CORNERS),
        }
    }
}
