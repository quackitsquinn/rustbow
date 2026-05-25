//! Main entry point for rustbow
use std::collections::HashMap;

use clap::Parser;
use rustbow::{
    config::{Charset, CharsetTemplate, ColorConfigModifier, RustBowConfig, RustBowConfigModifier},
    run,
};

/// RustBow: A colorful terminal animation of random characters.
#[derive(Parser)]
struct Args {
    #[clap(flatten)]
    charset: CharsetArgs,
    /// Foreground color configuration in the format "key:value,key:value,...".
    ///
    /// Valid keys are the following:
    ///
    /// - `rate`: the change rate of the color (float)
    ///
    /// - `h`: the initial hue of the color (float)
    ///
    /// - `s`: the saturation of the color (float)
    ///
    /// - `l`: the lightness of the color (float)
    #[clap(long = "fg")]
    fg_color_config: Option<String>,
    /// Background color configuration in the format "key:value,key:value,...".
    ///
    /// Valid keys are the same as for foreground color configuration.
    #[clap(long = "bg")]
    bg_color_config: Option<String>,

    /// The speed of the animation. In the format of <fps>:<chars_per_frame>.
    /// For example, `--speed 60:5` means to run the animation at 60 frames per second and print 5 characters per frame.
    ///
    /// Zero or negative fps means to run as fast as possible. If chars_per_frame is zero, it defaults to 3.
    #[clap(long = "speed")]
    #[clap(value_parser = parse_speed)]
    speed: Option<(f32, u32)>,
}

impl Args {
    pub fn to_modifier(&self) -> anyhow::Result<RustBowConfigModifier> {
        let fg = self
            .fg_color_config
            .as_deref()
            .map(parse_inline_color_str)
            .transpose()?
            .unwrap_or_default();

        let bg = self
            .bg_color_config
            .as_deref()
            .map(parse_inline_color_str)
            .transpose()?;

        let charset = match (
            self.charset.template.as_ref(),
            self.charset.charset.as_ref(),
        ) {
            (Some(template), None) => Some(template.to_charset()),
            (None, Some(charset)) => Some(Charset::owned(charset.chars().collect())),
            (None, None) => None,
            (Some(_), Some(_)) => unreachable!(),
        };

        Ok(RustBowConfigModifier {
            charset,
            foreground_config: Some(fg),
            background_config: bg,
            frames_per_second: self.speed.map(|(fps, _)| fps),
            chars_per_frame: self.speed.map(|(_, cpf)| cpf as usize),
        })
    }
}

fn parse_inline_color_str(cstr: &str) -> anyhow::Result<ColorConfigModifier> {
    let parts = cstr.split(',').map(str::trim);

    let mut attribs = HashMap::new();
    for part in parts {
        let mut kv = part.splitn(2, ':').map(str::trim);
        let key = kv
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid color config string: {cstr}"))?;
        match key {
            "rate" | "h" | "s" | "l" => {}
            _ => anyhow::bail!("Unknown color config key `{key}` in `{cstr}`"),
        }
        let value = kv
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid color config string: {cstr}"))?;
        attribs.insert(key, value);
    }

    let try_get_float = |key: &str| -> anyhow::Result<Option<f32>> {
        Ok(attribs.get(key).map(|v| v.parse::<f32>()).transpose()?)
    };

    Ok(ColorConfigModifier {
        change_rate: try_get_float("rate")?,
        initial_hue: try_get_float("h")?,
        saturation: try_get_float("s")?,
        lightness: try_get_float("l")?,
    })
}

fn parse_speed(speed_str: &str) -> anyhow::Result<(f32, u32)> {
    let mut parts = speed_str.split(':').map(str::trim);
    let fps_str = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid speed string: {speed_str}"))?;
    let chars_per_frame_str = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid speed string: {speed_str}"))?;

    let fps = fps_str.parse::<f32>()?.max(0.0);
    let chars_per_frame = chars_per_frame_str.parse::<u32>()?;

    Ok((fps, chars_per_frame))
}

#[derive(Parser, Clone)]
#[group(multiple = false)]
struct CharsetArgs {
    /// The template name to use instead of random characters.
    #[clap(long, group = "charset_template")]
    template: Option<CharsetTemplate>,
    /// A string of characters to use instead of random characters. Default is "@#$%&?".
    #[clap(long, group = "charset_template")]
    charset: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let config = RustBowConfig::default();
    let arg_modifier = Args::parse().to_modifier()?;
    let config = config.modify_with(&arg_modifier);

    println!("Running RustBow with config: {:#?}", config);

    run(&config)
}
