//! Main entry point for rustbow
use clap::Parser;
use rustbow::{
    config::{RustBowConfig, RustBowConfigModifier},
    run,
};

fn main() -> anyhow::Result<()> {
    let config = RustBowConfig::default();
    let arg_modifier = RustBowConfigModifier::parse();
    let config = config.modify_with(&arg_modifier);
    run(&config)
}
