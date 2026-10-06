use std::{any::Any, fmt::Debug};

/// A marker trait for types that can be attached to a node as an extension.
///
/// An extension is additional, type-distinguished data attached to a node
/// beyond its core [`NodeData`], allowing nodes to be augmented with custom
/// behavior or state. Any type that is `'static` (via [`Any`]) and implements
/// [`Debug`] automatically qualifies as an `Extension`.
///
/// This trait has no required methods; it exists to bound APIs like
/// [`AddExtensionToNode`] to types that are suitable for attachment to nodes.
pub trait Extension: Debug + Any {}
