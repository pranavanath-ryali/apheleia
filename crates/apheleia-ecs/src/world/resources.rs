use super::*;

impl World {
    /// Add and register given [`Resource`]
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource itself that should be added to [`World`]
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyResource {
    ///     value: i32
    /// }
    /// impl Resource for MyResource {}
    ///
    /// world.add_resource(MyResource { value: 123 });
    /// ```
    #[inline]
    pub fn add_resource<R: Resource>(&mut self, resource: R) {
        self.resource_store.add_resource(Box::new(resource));
    }

    /// Add and register the given [`Box<Resource>`]
    ///
    /// # Arguments
    ///
    /// * `resource` - The boxed resource itself that should be added to [`World`]
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyResource {
    ///     value: i32
    /// }
    /// impl Resource for MyResource {}
    ///
    /// world.add_resource(Box::new(MyResource { value: 123 }));
    /// ```
    #[inline]
    pub fn add_resource_boxed<R: Resource>(&mut self, resource: Box<R>) {
        self.resource_store.add_resource(resource);
    }

    /// Get reference of [`Resource`] that was registered to [`World`]
    #[inline]
    pub fn get_resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resource_store.get_resource::<R>()
    }

    /// Get mutable reference of [`Resource`] that was registered to [`World`]
    #[inline]
    pub fn get_resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resource_store.get_resource_mut::<R>()
    }
}
