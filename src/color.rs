use core::fmt;

/// The location for applying a color
pub enum Canvas {
    Background,
    Foreground,
}

/// Generates ANSI escape sequences for a specific kind of color
pub trait FormatColor {
    /// Apply the color
    fn prelude(&self, f: &mut fmt::Formatter, canvas: &Canvas) -> fmt::Result;

    /// Undo the color application
    #[allow(unused)]
    fn epilogue(&self, f: &mut fmt::Formatter, canvas: &Canvas) -> fmt::Result {
        f.write_str("\x1B[0m")
    }
}

/// Something which has been colored
pub struct Colored<T, TFormatColor: FormatColor> {
    t: T,
    format_color: TFormatColor,
    canvas: Canvas
}

/// Adds a foreground or background color
pub trait Colorable<TFormatColor: FormatColor>: Sized {
    /// Adds the given background color
    fn bg(self, format_color: TFormatColor) -> Colored<Self, TFormatColor>;

    /// Adds the given foreground color
    fn fg(self, format_color: TFormatColor) -> Colored<Self, TFormatColor>;
}

impl<T, TFormatColor: FormatColor> Colorable<TFormatColor> for T {
    fn bg(self, format_color: TFormatColor) -> Colored<Self, TFormatColor> {
        Colored {
            t: self,
            format_color,
            canvas: Canvas::Background
        }
    }

    fn fg(self, format_color: TFormatColor) -> Colored<Self, TFormatColor> {
        Colored {
            t: self,
            format_color,
            canvas: Canvas::Foreground
        }
    }
}

macro_rules! impl_me {
    ($bound:path) => {
        impl<T: $bound, TFormatColor: FormatColor> $bound for Colored<T, TFormatColor> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.format_color.prelude(f, &self.canvas)
                    .and_then(|_| self.t.fmt(f))
                    .and_then(|_| self.format_color.epilogue(f, &self.canvas))
            }
        }
    };
}

impl_me!(fmt::Binary);
impl_me!(fmt::Debug);
impl_me!(fmt::Display);
impl_me!(fmt::LowerExp);
impl_me!(fmt::LowerHex);
impl_me!(fmt::Octal);
impl_me!(fmt::Pointer);
impl_me!(fmt::UpperExp);
impl_me!(fmt::UpperHex);
