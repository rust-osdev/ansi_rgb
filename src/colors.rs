use crate::Canvas;
use crate::FormatColor;
use core::fmt;
use rgb::RGB8;

impl FormatColor for RGB8 {
    fn prelude(&self, f: &mut fmt::Formatter, canvas: crate::Canvas) -> fmt::Result {
        match canvas {
            Canvas::Foreground => write!(f, "\x1B[38;2;{};{};{}m", self.r, self.g, self.b),
            Canvas::Background => write!(f, "\x1B[48;2;{};{};{}m", self.r, self.g, self.b),
        }
    }
}

/// Makes <code style="color: black; background: #FFFFFF">white</code>
pub const fn white() -> RGB8 {
    RGB8::new(255, 255, 255)
}

/// Makes <code style="color: white; background: #000000">black</code>
pub const fn black() -> RGB8 {
    RGB8::new(0, 0, 0)
}

/// Makes <code style="color: black; background: #FF0000">red</code>
pub const fn red() -> RGB8 {
    RGB8::new(255, 0, 0)
}

/// Makes <code style="color: black; background: #FF8000">orange</code>
pub const fn orange() -> RGB8 {
    RGB8::new(255, 128, 0)
}

/// Makes <code style="color: black; background: #FFFF00">yellow</code>
pub const fn yellow() -> RGB8 {
    RGB8::new(255, 255, 0)
}

/// Makes <code style="color: black; background: #80FF00">yellow green</code>
pub const fn yellow_green() -> RGB8 {
    RGB8::new(128, 255, 0)
}

/// Makes <code style="color: black; background: #00FF00">green</code>
pub const fn green() -> RGB8 {
    RGB8::new(0, 255, 0)
}

/// Makes <code style="color: black; background: #00FF80">green cyan</code>
pub const fn green_cyan() -> RGB8 {
    RGB8::new(0, 255, 128)
}

/// Makes <code style="color: black; background: #00FFFF">cyan</code>
pub const fn cyan() -> RGB8 {
    RGB8::new(0, 255, 255)
}

/// Makes <code style="color: white; background: #0080FF">cyan blue</code>
pub const fn cyan_blue() -> RGB8 {
    RGB8::new(0, 128, 255)
}

/// Makes <code style="color: white; background: #0000FF">blue</code>
pub const fn blue() -> RGB8 {
    RGB8::new(0, 0, 255)
}

/// Makes <code style="color: white; background: #8000FF">blue magenta</code>
pub const fn blue_magenta() -> RGB8 {
    RGB8::new(128, 0, 255)
}

/// Makes <code style="color: black; background: #FF00FF">magenta</code>
pub const fn magenta() -> RGB8 {
    RGB8::new(255, 0, 255)
}

/// Makes <code style="color: black; background: #FF0080">magenta pink</code>
pub const fn magenta_pink() -> RGB8 {
    RGB8::new(255, 0, 128)
}

/// Makes <code style="color: white"><span style="background: #B00">R</span><span style="background: #0B0">G</span><span style="background: #00B">B</span></code>
pub const fn rgb(r: u8, g: u8, b: u8) -> RGB8 {
    RGB8::new(r, g, b)
}

/// A 3-bit color type
///
/// The exact colors usually depend on the terminal color scheme.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl FormatColor for Color3 {
    fn prelude(&self, f: &mut fmt::Formatter, canvas: crate::Canvas) -> fmt::Result {
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl FormatColor for Color4 {
    fn prelude(&self, f: &mut fmt::Formatter, canvas: crate::Canvas) -> fmt::Result {
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

impl From<Color3> for Color4 {
    fn from(color3: Color3) -> Self {
        Self::new(color3, false)
    }
}

/// Error type indicating an input argument was out of bounds
#[derive(Debug)]
pub struct OutOfBoundsError();

/// An 8-bit color
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Color8 {
    byte: u8
}

impl Color8 {
    /// Create a `Color8` using the given 8-bit color code.
    /// See [https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit)
    pub const fn new(byte: u8) -> Self {
        Self { byte }
    }

    /// New RGB color, each channel in range `0..6`
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Result<Self, OutOfBoundsError> {
        if r < 6 && g < 6 && b < 6 {
            Ok(Self {
                byte: 16 + r * 36 + g * 6 + b,
            })
        } else {
            Err(OutOfBoundsError())
        }
    }

    /// New gray color in range `0..24`
    pub fn new_gray(gray: u8) -> Result<Self, OutOfBoundsError> {
        if gray < 24 {
            Ok(Self { byte: 232 + gray })
        } else {
            Err(OutOfBoundsError())
        }
    }
}

impl FormatColor for Color8 {
    fn prelude(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result {
        write!(
            f,
            "\x1B[{};5;{}m",
            match canvas {
                Canvas::Foreground => 38,
                Canvas::Background => 48
            },
            self.byte
        )
    }
}

impl From<Color3> for Color8 {
    fn from(color3: Color3) -> Self {
        Self::new(color3 as u8)
    }
}

impl From<Color4> for Color8 {
    fn from(color4: Color4) -> Self {
        Self::new(
            (color4.color3 as u8) + match color4.bright {
                true => 8,
                false => 0
            }
        )
    }
}
