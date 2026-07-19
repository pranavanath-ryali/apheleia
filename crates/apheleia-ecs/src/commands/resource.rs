use crate::{traits::{context_command::ContextCommand, resource::Resource}, world::World};

/// A [`ContextCommand`] that adds a [`Resource`] of type `R` to the [`World`].
///
/// Wraps a resource value, inserting it into the world when executed.
#[derive(Debug)]
pub struct AddResource<R: Resource>(pub R);

impl<R: Resource> AddResource<R> {
    /// Creates a new boxed [`AddResource`] command for the given resource.
    ///
    /// # Arguments
    ///
    /// * `res` - The resource instance to add to the world.
    pub fn new(res: R) -> Box<Self> {
        Box::new(Self(res))
    }
}

impl<R: Resource> ContextCommand for AddResource<R> {
    /// Executes the command, adding the resource to the [`World`].
    fn execute(self: Box<Self>, world: &mut World) {
        world.add_resource(self.0);
    }
}
