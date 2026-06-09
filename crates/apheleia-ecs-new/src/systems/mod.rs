pub mod system;
pub(crate) mod store;
pub mod stages;

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use crate::{constants::PRE_STAGE, systems::{stages::SystemRunStage, store::SystemStore, system::SystemParam}};

    struct TestParam {
        value: f32
    }
    impl SystemParam for TestParam {
        unsafe fn fetch(world: *mut crate::world::World) -> Option<Self> {
            Some(TestParam { value: 123.0 })
        }
    }

    impl Deref for TestParam {
        type Target = f32;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl DerefMut for TestParam {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }

    fn a_system(param: TestParam) {
        println!("WHY DOES THIS WORK: {}", *param);
    }

    #[test]
    fn test_systems() {
        let mut store = SystemStore::default();
        store.add_system(SystemRunStage::Update, PRE_STAGE, a_system);
    }
}
