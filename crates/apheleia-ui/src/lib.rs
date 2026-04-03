pub mod builder;
pub mod contexts;
pub mod extensions;
pub mod id_generator;
pub mod node;
pub mod resources;
pub mod rootnode;
pub mod systems;
pub mod types;

use std::{error::Error, fs::OpenOptions};

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
