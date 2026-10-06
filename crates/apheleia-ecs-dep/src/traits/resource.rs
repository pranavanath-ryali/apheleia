use std::{any::Any, fmt::Debug};

/// A marker trait for types that can be stored as resources in the [`World`].
///
/// A resource is a singleton-like piece of data (per type) attached to the
/// world, distinct from per-node data. Any type that is `'static` (via
/// [`Any`]) and implements [`Debug`] automatically qualifies as a `Resource`.
///
/// This trait has no required methods; it exists to bound APIs like
/// [`AddResource`] to types that are suitable for storage as world resources.
pub trait Resource: Any + Debug {}
