//! Configuration for Rustbow.
use crate::ArcSlice;

/// Configuration for Rustbow.
pub struct RustBowConfig {
    /// A string of characters to use instead of random characters. Default is "@#$%&?".
    pub charset: ArcSlice<char>,
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
                .as_ref()
                .map(|f| ArcSlice::from(f.chars().collect::<Vec<char>>()))
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
            charset: ArcSlice::from(vec!['@', '#', '$', '%', '&', '?']),
            change_rate: 0.001,
            saturation: 1.0,
            lightness: 0.8,
        }
    }
}

/// A modifier for the RustBowConfig. This is used to modify the config with command line arguments.
#[derive(clap::Parser, serde_derive::Deserialize, serde_derive::Serialize, Debug)]
pub struct RustBowConfigModifier {
    /// A string of characters to use instead of random characters. Default is "@#$%&?".
    #[clap(long, short = 'r')]
    pub charset: Option<String>,
    /// The amount to increment the hue by. Default is 0.001.
    #[clap(long, short = 'c')]
    pub change_rate: Option<f32>,
    /// The saturation of the outputted colors, between 0 and 1. Default is 1.
    #[clap(long, short = 's')]
    pub saturation: Option<f32>,
    /// The value of the outputted colors, between 0 and 1. Default is 1.
    #[clap(long, short = 'v')]
    pub value: Option<f32>,
}
