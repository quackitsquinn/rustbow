//! Main entry point for rustbow
use clap::{Parser, ValueEnum};
use rustbow::{
    config::{Charset, CharsetTemplate, RustBowConfig, RustBowConfigModifier},
    run,
};

#[derive(Parser)]
struct Args {
    #[clap(flatten)]
    charset: CharsetArgs,
    /// The amount to increment the hue by per character. Default is 0.001.
    #[clap(long, short = 't')]
    change_rate: Option<f32>,
    /// The saturation of the outputted colors, between 0 and 1. Default is 1.
    #[clap(long, short = 's')]
    saturation: Option<f32>,
    /// The value of the outputted colors, between 0 and 1. Default is 1.
    #[clap(long, short = 'l')]
    lightness: Option<f32>,
}

impl Args {
    pub fn to_modifier(&self) -> RustBowConfigModifier {
        match (
            self.charset.template.as_ref(),
            self.charset.charset.as_ref(),
        ) {
            (Some(template), None) => RustBowConfigModifier {
                charset: Some(template.to_charset()),
                change_rate: self.change_rate,
                saturation: self.saturation,
                value: self.lightness,
            },
            (None, Some(charset)) => RustBowConfigModifier {
                charset: Some(Charset::owned(charset.chars().collect())),
                change_rate: self.change_rate,
                saturation: self.saturation,
                value: self.lightness,
            },
            (None, None) => RustBowConfigModifier {
                charset: None,
                change_rate: self.change_rate,
                saturation: self.saturation,
                value: self.lightness,
            },
            (Some(_), Some(_)) => unreachable!(),
        }
    }
}

#[derive(Parser, Clone)]
#[group(multiple = false)]
struct CharsetArgs {
    /// The template name to use instead of random characters. use <TODO> to list the templates
    #[clap(long, group = "charset_template")]
    template: Option<CharsetTemplate>,
    /// A string of characters to use instead of random characters. Default is "@#$%&?".
    #[clap(long, group = "charset_template")]
    charset: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let config = RustBowConfig::default();
    let arg_modifier = Args::parse().to_modifier();
    let config = config.modify_with(&arg_modifier);
    println!("config: {:#?}", config);
    run(&config)
}
