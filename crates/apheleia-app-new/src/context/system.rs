use apheleia_ecs_new::world::World;

use crate::app::App;

pub struct SystemContext {
    app: *mut App,
}
impl SystemContext {
    pub fn new(app: *mut App) -> Self {
        Self {
            app
        }
    }
}
