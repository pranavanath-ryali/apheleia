use apheleia_app::app::App;

use crate::my_definer::CustomDefiner;

mod my_definer {
    use apheleia_app::{context::system::SystemContext, node_definer::NodeDefiner};
    use apheleia_ecs::{constants::STAGE, resources::Resource, system_params::resource::{Res, ResMut}};

    #[derive(Debug)]
    pub struct CustomDefiner;
    impl NodeDefiner for CustomDefiner {
        fn setup(&mut self, ctx: &mut apheleia_app::context::node::NodeContext) {
            ctx.add_system(apheleia_ecs::types::SystemRunStage::Update, STAGE, do_stuff);
        }
    }

    #[derive(Debug)]
    pub struct CustomResource;
    impl Resource for CustomResource {}

    fn do_stuff(res: ResMut<CustomResource>, mut ctx: SystemContext) {}
}

fn main() {
    App::new()
        .build_node(|builder| builder.node(CustomDefiner))
        .run();
}
