pub mod builder;
pub mod contexts;
pub mod dirty_tracker;
pub mod extensions;
pub mod id_generator;
pub mod node;
pub mod resources;
pub mod root;
pub mod systems;
pub mod types;
pub mod world;

use std::{error::Error, fs::OpenOptions};

pub use apheleia_core::types::vector;
pub use crossterm::event::*;
use fern::Dispatch;
use log::info;

pub fn setup_logger() -> Result<(), Box<dyn Error>> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")?;

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(log_file)
        .apply()?;

    info!("Log Started");
    Ok(())
}
