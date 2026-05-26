//! A terminal-based rainbow generator.
use std::{
    io::{BufWriter, Write},
    sync::mpsc,
    time::Duration,
};

use color::{Hsl, OpaqueColor, Rgba8};
use crossterm::{execute, queue, style::Color};
use rand::{rngs::ThreadRng, seq::IndexedRandom, RngExt};

use crate::{
    color_gen::{ColorGenerator, HueShiftGenerator},
    config::{Charset, RustBowConfig},
};

pub mod color_gen;
pub mod config;

struct Renderer {
    rng: ThreadRng,
    foreground_generator: Box<dyn ColorGenerator>,
    background_generator: Option<Box<dyn ColorGenerator>>,
    charset: Charset,
    term_dims: (u16, u16),
}

impl Renderer {
    pub fn new(config: &RustBowConfig, term_dims: (u16, u16)) -> Self {
        Self {
            rng: rand::rng(),
            foreground_generator: Box::new(HueShiftGenerator::new(
                config.foreground.change_rate,
                OpaqueColor::new([
                    config.foreground.initial_hue,
                    config.foreground.saturation * 100.,
                    config.foreground.lightness * 100.,
                ]),
            )),
            background_generator: config.background.map(|bg| {
                let gen = HueShiftGenerator::new(
                    bg.change_rate,
                    OpaqueColor::new([bg.initial_hue, bg.saturation * 100., bg.lightness * 100.]),
                );
                Box::new(gen) as Box<dyn ColorGenerator>
            }),

            charset: config.charset.clone(),
            term_dims,
        }
    }

    pub fn next_char(&mut self) -> char {
        *self.charset.chars.choose(&mut self.rng).unwrap()
    }

    pub fn next_pos(&mut self) -> (u16, u16) {
        let x = self.rng.random_range(0..self.term_dims.0);
        let y = self.rng.random_range(0..self.term_dims.1);
        (x, y)
    }

    pub fn output_char(&mut self, stdout: &mut impl Write) -> anyhow::Result<()> {
        use crossterm::{
            cursor::MoveTo,
            style::{Print, SetBackgroundColor, SetForegroundColor},
        };

        let chr = self.next_char();
        let pos = self.next_pos();
        let color = self.foreground_generator.get(&mut self.rng, pos);
        queue!(
            stdout,
            MoveTo(pos.0, pos.1),
            SetForegroundColor(Color::Rgb {
                r: color.r,
                g: color.g,
                b: color.b
            }),
        )?;
        if let Some(gen) = &mut self.background_generator {
            let bg_color = gen.get(&mut self.rng, pos);
            queue!(
                stdout,
                SetBackgroundColor(Color::Rgb {
                    r: bg_color.r,
                    g: bg_color.g,
                    b: bg_color.b
                }),
            )?;
        }
        queue!(stdout, Print(chr))?;
        Ok(())
    }

    pub fn output_frame(
        &mut self,
        stdout: &mut impl Write,
        chars_per_frame: usize,
    ) -> anyhow::Result<()> {
        for _ in 0..chars_per_frame {
            self.output_char(stdout)?;
        }
        Ok(())
    }
}

/// Runs the main loop of the program, generating random colors and characters and printing them to the terminal.
pub fn run(config: &RustBowConfig) -> anyhow::Result<()> {
    let mut generator = Renderer::new(config, crossterm::terminal::size().unwrap_or((20, 20)));
    let mut stdout = BufWriter::new(std::io::stdout());

    let (closer, should_close) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        let _ = closer.send(());
        // wait a few seconds, if we don't exit something has gone wrong and we should panic.
        std::thread::sleep(Duration::from_secs(2));
        let mut stdout = std::io::stdout();
        let _ = execute!(
            stdout,
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );
        println!("Failed to exit cleanly after Ctrl-C; exiting forcefully.");
        std::process::exit(1);
    })?;

    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    stdout.flush()?;

    let frame_duration = if config.frames_per_second > 0. {
        Duration::from_secs_f32(1. / config.frames_per_second)
    } else {
        Duration::ZERO
    };

    let mut now = std::time::Instant::now();
    let mut next = now + frame_duration;
    while should_close.try_recv().is_err() {
        generator.output_frame(&mut stdout, config.chars_per_frame)?;
        stdout.flush()?;

        now = std::time::Instant::now();
        if next > now {
            std::thread::sleep(next - now);
        }
        next = now + frame_duration;
    }

    execute!(
        stdout,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    Ok(())
}
