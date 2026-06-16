use std::fmt::Debug;

pub(crate) mod tracker;

/// A marker and configuration trait for strictly typed events.
///
/// [`EventTrait`] ensures that any implementing type is safely printable for debugging
/// purposes and possesses a static lifetime.
///
/// ### Requirements
///
/// * **`Debug`**: Allows the event to be formatted using `{:?}` or `{:#?}` for logging and tracing.
/// * **`'static`**: To be used with `std::any::TypeId`
///
/// # Examples
///
/// ```rust
/// #[derive(Debug)]
/// pub struct ButtonClicked;
/// impl EventTrait for ButtonClicked {}
///
/// let node_id = world.create_node();
///
/// world.add_local_event(node_id, ButtonClicked);
/// ```
pub trait EventTrait: Debug + 'static {}

/// A marker event to tell the [`App`] to redraw the UI for nodes with this event
#[derive(Debug)]
pub struct RenderDirty;
impl EventTrait for RenderDirty {}
