// This exists to purely reexport all necessary and usable components of this crate
// TODO: Rewrite this entire crate

pub use apheleia_core::{
    Color, buffer, rich_strings,
    style::{Style, StyleFlags},
    types::Vec2,
};
pub use apheleia_macros::{Extension as ExtensionMacro, Resource as ResourceMacro};
pub use apheleia_ui::{
    KeyCode,
    contexts::{commands::*, node, system, traits},
    extensions::traits::Extension,
    node::EmptyNode,
    node::traits::NodeTrait,
    resources::traits::Resource,
    root, setup_logger,
    types::*,
};
pub use apheleia_widgets::*;
