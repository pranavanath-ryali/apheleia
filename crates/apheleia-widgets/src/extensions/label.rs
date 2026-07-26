use apheleia_core::rich_strings::RichString;
use apheleia_ecs::traits::extension::Extension;

#[derive(Default, Debug)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Default, Debug)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Debug)]
pub struct LabelExtension {
    pub text: RichString,

    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}
impl Extension for LabelExtension {}
impl Default for LabelExtension {
    fn default() -> Self {
        Self {
            text: RichString::new(""),
            horizontal_alignment: Default::default(),
            vertical_alignment: Default::default(),
        }
    }
}
