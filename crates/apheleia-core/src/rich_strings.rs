use crate::style::Style;

pub struct RichString {
    text: String,
}

impl RichString {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
    pub fn to_rich(text: &str, style: Style) -> Self {
        Self {
            text: format!("{}", text).to_string(),
        }
    }

    fn get_markup(style: Style) -> String {
        let markup: String = "".to_string();

        markup
    }
}
