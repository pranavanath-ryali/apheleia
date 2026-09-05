#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Grapheme {
    // Ascii(u8),
    Char(char),
    Width(char),
    // Extended, // TODO: todo
}

