pub struct RichString {
    text: String,
}

impl RichString {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}
