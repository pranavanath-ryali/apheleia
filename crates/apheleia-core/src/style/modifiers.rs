bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Modifiers: u8 {
        const NONE              = 0;
        const BOLD              = 1 << 0;
        const DOUBLE_UNDERLINE  = 1 << 1;
        const ITALIC            = 1 << 2;
        const UNDERLINE         = 1 << 3;
        const BLINK             = 1 << 4;
        const REVERSE           = 1 << 5;
        const CONCEAL           = 1 << 6;
        const STRIKETHROUGH     = 1 << 7;
    }
}
use bitflags::bitflags;
