//! A terminal-based rainbow generator.
use std::{
    io::{BufReader, BufWriter, Write},
    sync::mpsc,
    thread::{self, Thread},
    time::Duration,
};

use crossterm::{
    event::{self, Event},
    execute,
};

use crate::{config::RustBowConfig, frame::FrameTracker, render::Renderer};

pub mod color_gen;
pub mod config;
mod frame;
pub mod parser;
mod render;

const WATCHDOG_TERMINATOR: Duration = Duration::from_secs(2);

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
    crossterm::terminal::enable_raw_mode()?;
    Ok(())
}

fn restore_terminal(term: &mut impl Write) -> anyhow::Result<()> {
    execute!(
        term,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show,
        crossterm::style::ResetColor
    )?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn install_handler() -> anyhow::Result<mpsc::Receiver<()>> {
    let (ctrlc_closer, should_close) = mpsc::channel::<()>();
    let keyboard_closer = ctrlc_closer.clone();

    ctrlc::set_handler(move || {
        let _ = ctrlc_closer.send(());
        // wait a few seconds, if we don't exit something has gone wrong and we should just exit forcefully
        std::thread::sleep(WATCHDOG_TERMINATOR);
        watchdog_terminator("Ctrl-C");
    })?;

    thread::spawn(move || {
        while !keyboard_input() {
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = keyboard_closer.send(());
        std::thread::sleep(WATCHDOG_TERMINATOR);
        watchdog_terminator("keyboard input");
    });

    Ok(should_close)
}

fn watchdog_terminator(event: &str) {
    let mut stdout = std::io::stdout();
    let _ = restore_terminal(&mut stdout);
    println!("Failed to exit cleanly after {event}; exiting forcefully.");
    std::process::exit(1);
}

fn keyboard_input() -> bool {
    event::poll(Duration::from_millis(100)).unwrap_or(false)
        && matches!(event::read(), Ok(Event::Key(_)))
}
