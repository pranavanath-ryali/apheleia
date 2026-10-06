use core::fmt::Debug;

use crate::world::World;

/// A command that can be applied to the [`World`] to mutate its state.
///
/// `ContextCommand`s encapsulate a single, deferred mutation (e.g. adding a
/// node, setting data, registering a system) as a boxed, debuggable value.
/// This allows commands to be queued up and executed later against the
/// world, rather than mutating it immediately.
pub trait ContextCommand: Debug {
    /// Consumes the boxed command, applying its effect to the given [`World`].
    fn execute(self: Box<Self>, world: &mut World);
}
