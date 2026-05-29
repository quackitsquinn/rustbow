//! A terminal-based rainbow generator.
use std::{
    io::{BufWriter, Write},
    sync::mpsc,
    time::Duration,
};

use crossterm::execute;

use crate::{config::RustBowConfig, frame::FrameTracker, render::Renderer};

pub mod color_gen;
pub mod config;
mod frame;
pub mod parser;
mod render;

const CTRLC_WATCHDOG_TIMEOUT: Duration = Duration::from_secs(2);

/// Runs the main loop of the program, generating random colors and characters and printing them to the terminal.
pub fn run(config: &RustBowConfig) -> anyhow::Result<()> {
    let mut renderer = Renderer::new(
        config,
        crossterm::terminal::size().unwrap_or((20, 20)),
        config.foreground.build_generator(),
        config.background.as_ref().map(|bg| bg.build_generator()),
    );
    let mut stdout = BufWriter::new(std::io::stdout());

    let should_close = install_handler()?;
    setup_terminal(&mut stdout)?;

    let mut tracker = FrameTracker::new(config.frames_per_second);
    while should_close.try_recv().is_err() {
        renderer.output_frame(&mut stdout, config.chars_per_frame)?;
        stdout.flush()?;
        tracker.end_frame();
    }

    restore_terminal(&mut stdout)?;
    Ok(())
}

fn setup_terminal(term: &mut impl Write) -> anyhow::Result<()> {
    execute!(
        term,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    Ok(())
}

fn restore_terminal(term: &mut impl Write) -> anyhow::Result<()> {
    execute!(
        term,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    Ok(())
}

fn install_handler() -> anyhow::Result<mpsc::Receiver<()>> {
    let (closer, should_close) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        let _ = closer.send(());
        // wait a few seconds, if we don't exit something has gone wrong and we should just exit forcefully
        std::thread::sleep(CTRLC_WATCHDOG_TIMEOUT);
        ctrlc_watchdog_terminator();
    })?;

    Ok(should_close)
}

fn ctrlc_watchdog_terminator() {
    let mut stdout = std::io::stdout();
    let _ = restore_terminal(&mut stdout);
    println!("Failed to exit cleanly after Ctrl-C; exiting forcefully.");
    std::process::exit(1);
}
