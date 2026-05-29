use std::io::Write;

use crossterm::{queue, style::Color};
use rand::{rngs::ThreadRng, seq::IndexedRandom, RngExt};

use crate::{
    color_gen::ColorGenerator,
    config::{Charset, RustBowConfig},
};

pub struct Renderer {
    rng: ThreadRng,
    foreground_generator: Box<dyn ColorGenerator>,
    background_generator: Option<Box<dyn ColorGenerator>>,
    charset: Charset,
    term_dims: (u16, u16),
}

impl Renderer {
    pub fn new(
        config: &RustBowConfig,
        term_dims: (u16, u16),
        foreground_generator: Box<dyn ColorGenerator>,
        background_generator: Option<Box<dyn ColorGenerator>>,
    ) -> Self {
        Self {
            rng: rand::rng(),
            foreground_generator,
            background_generator,
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
