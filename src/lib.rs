//! A terminal-based rainbow generator.
use std::{io::Write, ops::Add, sync::mpsc};

use color::{Hsl, OpaqueColor, ProphotoRgb, Rgba8};
use crossterm::{execute, queue, style::Color};
use rand::{
    distr::{Alphanumeric, SampleString},
    rngs::ThreadRng,
    seq::IndexedRandom,
    RngExt,
};

use crate::config::{Charset, RustBowConfig};

pub mod config;

pub(crate) type ArcSlice<T> = std::sync::Arc<[T]>;
pub(crate) type ArcStr = std::sync::Arc<str>;

struct Generator {
    rng: ThreadRng,
    foreground: OpaqueColor<Hsl>,
    background: Option<OpaqueColor<Hsl>>,
    fg_rate: f32,
    bg_rate: f32,
    charset: Charset,
    term_dims: (u16, u16),
}

impl Generator {
    pub fn new(config: &RustBowConfig, term_dims: (u16, u16)) -> Self {
        Self {
            rng: rand::rng(),
            foreground: OpaqueColor::new([
                config.foreground.initial_hue,
                config.foreground.saturation * 100.,
                config.foreground.lightness * 100.,
            ]),
            background: config.background.map(|bg| {
                OpaqueColor::new([bg.initial_hue, bg.saturation * 100., bg.lightness * 100.])
            }),
            fg_rate: config.foreground.change_rate,
            bg_rate: config
                .background
                .map(|bg| bg.change_rate)
                .unwrap_or(config.foreground.change_rate),

            charset: config.charset.clone(),
            term_dims,
        }
    }

    pub fn next_fg_color(&mut self) -> Rgba8 {
        self.foreground = self.foreground.map_hue(|h| (h + self.fg_rate) % 360.);
        self.foreground.to_rgba8()
    }

    pub fn next_bg_color(&mut self) -> Option<Rgba8> {
        self.background.as_mut().map(|bg| {
            *bg = bg.map_hue(|h| (h + self.bg_rate) % 360.);
            bg.to_rgba8()
        })
    }

    pub fn next_char(&mut self) -> char {
        *self.charset.chars.choose(&mut self.rng).unwrap()
    }

    pub fn next_pos(&mut self) -> (u16, u16) {
        let x = self.rng.random_range(0..self.term_dims.0);
        let y = self.rng.random_range(0..self.term_dims.1);
        (x, y)
    }
}

/// Runs the main loop of the program, generating random colors and characters and printing them to the terminal.
pub fn run(config: &RustBowConfig) -> anyhow::Result<()> {
    let mut generator = Generator::new(config, crossterm::terminal::size().unwrap_or((20, 20)));
    let mut stdout = std::io::stdout();

    let (closer, should_close) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        let _ = closer.send(());
    })?;

    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    while should_close.try_recv().is_err() {
        let color = generator.next_fg_color();
        let chr = generator.next_char();
        let pos = generator.next_pos();
        queue!(
            stdout,
            crossterm::cursor::MoveTo(pos.0, pos.1),
            crossterm::style::SetForegroundColor(Color::Rgb {
                r: color.r,
                g: color.g,
                b: color.b
            }),
        )?;
        if let Some(bg_color) = generator.next_bg_color() {
            queue!(
                stdout,
                crossterm::style::SetBackgroundColor(Color::Rgb {
                    r: bg_color.r,
                    g: bg_color.g,
                    b: bg_color.b
                }),
            )?;
        }
        queue!(stdout, crossterm::style::Print(chr))?;
        stdout.flush()?;
    }

    execute!(
        stdout,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    Ok(())
}
