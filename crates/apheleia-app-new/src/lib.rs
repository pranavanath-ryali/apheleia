use std::{error::Error, fs::OpenOptions};

use fern::Dispatch;
use log::info;

pub mod app;
pub mod into_resource;
pub mod tag;
pub mod commands;
pub mod builder;
pub mod node_definer;
pub mod context;
pub mod params;

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

    info!("Log started");
    Ok(())
}
