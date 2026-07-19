use std::fmt::Debug;

/// A marker and configuration trait for strictly typed tags.
///
/// [`TagTrait`] ensures that any implementing type is safely printable for debugging 
/// purposes and possesses a static lifetime.
///
/// ### Requirements
///
/// * **`Debug`**: Allows the tag to be formatted using `{:?}` or `{:#?}` for logging and tracing.
/// * **`'static`**: To be used with `std::any::TypeId`
///
/// # Examples
///
/// ```rust
/// #[derive(Debug)]
/// pub struct MyButtonTag;
/// impl TagTrait for MyButtonTag {}
///
/// let node_id = world.create_node();
///
/// world.tag_node(node_id, MyButtonTag);
/// ```
pub trait TagTrait : Debug + 'static {}
