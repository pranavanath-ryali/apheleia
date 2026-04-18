// This exists to purely reexport all necessary and usable components of this crate

pub use apheleia_core::{
    Color, buffer, rich_strings,
    style::{Style, StyleFlags},
};
pub use apheleia_macros::{Extension, Resource};
pub use apheleia_ui::{
    contexts::{commands::*, node, system, traits},
    extensions::traits::Extension,
    node::traits::NodeTrait,
    resources::traits::Resource,
    root,
};
pub use apheleia_widgets::*;
