use crate::{stores::systems::{SystemStage, function_system::{IntoSystem, System}}, world::World};

impl World {
    #[inline]
    pub fn system_register<Params: 'static>(&mut self, stage: SystemStage, system: impl IntoSystem<Params>) {
        self.system_store.add_system(stage, system);
    }

    #[inline]
    pub fn system_register_boxed(&mut self, stage: SystemStage, system: Box<dyn System>) {
        self.system_store.add_system_boxed(stage, system);
    }

    #[inline]
    pub fn system_run(&mut self, stage: SystemStage) {
        let ptr = self as *mut World;
        self.system_store.run_stage(stage, ptr);
    }
}
