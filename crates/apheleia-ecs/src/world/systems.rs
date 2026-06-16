use super::*;

impl World {
    /// Add given function and register as system to [`World`]
    ///
    /// # Arguments
    ///
    /// * `stage` - The stage at which the system is run at and registered to
    /// * `priority` - The priority and order at which systems are run
    /// * `system` - The function itself that implements [`IntoSystem`]
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// fn test_system() {
    ///     println!("Hello from system");
    /// }
    ///
    /// world.add_system(SystemRunStage::Update, STAGE, test_system);
    ///
    /// ```
    #[inline]
    pub fn add_system<Params: 'static>(
        &mut self,
        stage: SystemRunStage,
        priority: u8,
        system: impl IntoSystem<Params>,
    ) {
        self.system_store.add_system(stage, priority, system);
    }
    
    /// Add given [`Box<dyn System>`] and register to [`World`]
    ///
    /// # Arguments
    ///
    /// * `stage` - The stage at which the system is run at and registered to
    /// * `priority` - The priority and order at which systems are run
    /// * `system` - The boxed value of [`System`] itself
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// fn test_system() {
    ///     println!("Hello from system");
    /// }
    ///
    /// world.add_system_boxed(SystemRunStage::Update, STAGE, IntoSystem::into_system(test_system));
    ///
    /// ```
    #[inline]
    pub fn add_system_boxed(
        &mut self,
        stage: SystemRunStage,
        priority: u8,
        system: Box<dyn System>,
    ) {
        self.system_store.add_system_boxed(stage, priority, system);
    }

    /// Run all [`System`]s registered in that stage and run in order of priority
    #[inline]
    pub fn run_systems_on_stage(&mut self, stage: SystemRunStage) {
        let ptr = self as *mut World;
        self.system_store.run_systems_for_stage(stage, ptr);
    }
}
