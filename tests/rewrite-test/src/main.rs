use std::arch::x86_64::_blci_u32;

use apheleia_app_new::{app::App, setup_logger};

const MY_TAG: usize = 0;

fn main() {
    setup_logger();
    App::new()
        .build_node(|builder| {
            builder
                .tag::<MY_TAG>()
        })
        .run();
}
