//! A terminal-based rainbow generator.
use std::{io::Write, ops::Add};

use color::{Hsl, OpaqueColor, ProphotoRgb, Rgba8};
use crossterm::{execute, queue, style::Color};
use rand::{rngs::ThreadRng, seq::IndexedRandom, RngExt};

use crate::config::RustBowConfig;

pub mod config;

pub(crate) type ArcSlice<T> = std::sync::Arc<[T]>;
pub(crate) type ArcStr = std::sync::Arc<str>;

struct Generator {
    rng: ThreadRng,
    curr: OpaqueColor<Hsl>,
    change_rate: f32,
    charset: ArcSlice<char>,
    term_dims: (u16, u16),
}

impl Generator {
    pub fn new(config: &RustBowConfig, term_dims: (u16, u16)) -> Self {
        Self {
            rng: rand::rng(),
            curr: OpaqueColor::new([0.0, config.saturation * 100., config.lightness * 100.]),
            change_rate: config.change_rate,
            charset: config.charset.clone(),
            term_dims,
        }
    }

    pub fn next_color(&mut self) -> Rgba8 {
        self.curr = self.curr.map_hue(|h| (h + self.change_rate) % 360.);
        self.curr.to_rgba8()
    }

    pub fn next_char(&mut self) -> char {
        *self.charset.choose(&mut self.rng).unwrap()
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

    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    loop {
        let color = generator.next_color();
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
            crossterm::style::Print(chr)
        )?;
        stdout.flush()?;
    }
}
