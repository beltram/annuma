#![allow(dead_code)]

pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn rgb(&self) -> String {
        let Color(r, g, b, t) = self;
        format!("{r:x}{g:x}{b:x}{t:x}")
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rgb())
    }
}

pub const BORDEAUX: Color = Color(136, 14, 79, 0);
pub const DARK_RED: Color = Color(165, 39, 20, 0);
pub const ORANGE: Color = Color(230, 81, 0, 0);
pub const DARK_YELLOW: Color = Color(249, 168, 37, 0);
pub const KAKI: Color = Color(129, 119, 23, 0);
pub const DARK_GREEN: Color = Color(9, 113, 56, 0);
pub const INDIGO: Color = Color(26, 35, 126, 0);
pub const DARK_BROWN: Color = Color(121, 85, 72, 0);
pub const BROWN: Color = Color(78, 52, 46, 0);
pub const VIOLET: Color = Color(78, 52, 46, 0);
pub const TURQUOISE: Color = Color(0, 96, 100, 0);
pub const DARK_BLUE: Color = Color(26, 35, 126, 0);
pub const YELLOW: Color = Color(251, 192, 45, 0);
pub const PURPLE: Color = Color(156, 39, 176, 0);
pub const BLACK: Color = Color(0, 0, 0, 0);
pub const GREY: Color = Color(117, 117, 117, 0);
pub const WHITE: Color = Color(255, 255, 255, 0);
