use crate::{Canvas, Color};

/// A 3-bit color type
///
/// The exact colors usually depend on the terminal color scheme.
#[derive(Copy, Clone, Debug)]
pub enum Color3 {
    BLACK = 0,
    RED = 1,
    GREEN = 2,
    YELLOW = 3,
    BLUE = 4,
    MAGENTA = 5,
    CYAN = 6,
    WHITE = 7,
}

impl Color for Color3 {
    fn prelude(&self, f: &mut core::fmt::Formatter, canvas: crate::Canvas) -> core::fmt::Result {
        match canvas {
            Canvas::Foreground => write!(f, "\x1B[{}m", 30 + *self as u8),
            Canvas::Background => write!(f, "\x1B[{}m", 40 + *self as u8),
        }
    }
}

/// A 4-bit color type
///
/// The different possibilities are available through associated constants;
/// their exact colors usually depend on the terminal color scheme. A 4-bit
/// color without its bright bit set is identical to its corresponding 3-bit
/// color (not necessarily the one with the same name).
#[derive(Copy, Clone, Debug)]
pub struct Color4 {
    color3: Color3,
    bright: bool,
}

impl Color4 {
    /// Build a Color4 based on [`Color3`] and a bright bit
    pub const fn new(color3: Color3, bright: bool) -> Self {
        Self { color3, bright }
    }

    pub const BLACK: Self = Self::new(Color3::BLACK, false);
    pub const RED: Self = Self::new(Color3::RED, false);
    pub const GREEN: Self = Self::new(Color3::GREEN, false);
    pub const YELLOW: Self = Self::new(Color3::YELLOW, false);
    pub const BLUE: Self = Self::new(Color3::BLUE, false);
    pub const MAGENTA: Self = Self::new(Color3::MAGENTA, false);
    pub const CYAN: Self = Self::new(Color3::CYAN, false);
    pub const LIGHT_GRAY: Self = Self::new(Color3::WHITE, false);
    pub const DARK_GRAY: Self = Self::new(Color3::BLACK, true);
    pub const BRIGHT_RED: Self = Self::new(Color3::RED, true);
    pub const BRIGHT_GREEN: Self = Self::new(Color3::GREEN, true);
    pub const BRIGHT_YELLOW: Self = Self::new(Color3::YELLOW, true);
    pub const BRIGHT_BLUE: Self = Self::new(Color3::BLUE, true);
    pub const BRIGHT_MAGENTA: Self = Self::new(Color3::MAGENTA, true);
    pub const BRIGHT_CYAN: Self = Self::new(Color3::CYAN, true);
    pub const WHITE: Self = Self::new(Color3::WHITE, true);
}

impl Color for Color4 {
    fn prelude(&self, f: &mut core::fmt::Formatter, canvas: crate::Canvas) -> core::fmt::Result {
        match canvas {
            Canvas::Foreground => write!(
                f,
                "\x1B[{}m",
                if self.bright { 90 } else { 30 } + self.color3 as u8
            ),
            Canvas::Background => write!(
                f,
                "\x1B[{}m",
                if self.bright { 100 } else { 40 } + self.color3 as u8
            ),
        }
    }
}
