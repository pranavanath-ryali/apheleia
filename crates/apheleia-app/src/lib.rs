use apheleia_core::{KeyEvent, MouseEvent, types::Vec2};
use std::{error::Error, fs::OpenOptions};

use fern::Dispatch;
use log::info;

pub mod app;
pub mod builder;
pub mod context;
pub mod events;
pub mod node_definer;
pub mod resources;
pub mod params;
pub mod systems;

pub mod types {
    use super::*;

    #[derive(Default)]
    pub enum EventData {
        Resize(Vec2),
        Keys(KeyEvent),
        Mouse(MouseEvent),

        FocusGained,
        FocusLost,

        #[default]
        None,
    }
}

