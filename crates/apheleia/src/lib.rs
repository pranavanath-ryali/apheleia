// This exists to purely reexport all necessary and usable components of this crate
// TODO: Rewrite this entire crate

pub use apheleia_core::{
    Color, buffer, rich_strings,
    style::{Style, StyleFlags},
    types::Vector2,
};
pub use apheleia_macros::{Extension, Resource};
pub use apheleia_ui::{
    contexts::{commands::*, node, system, traits},
    extensions::traits::Extension,
    node::traits::NodeTrait,
    resources::traits::Resource,
    root, setup_logger,
};
pub use apheleia_widgets::*;
