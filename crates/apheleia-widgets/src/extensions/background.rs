use apheleia_core::Color;
use apheleia_ecs::traits::extension::Extension;

#[derive(Debug)]
pub struct BackgroundExtension {
    pub color: Color
}
impl Extension for BackgroundExtension {}
