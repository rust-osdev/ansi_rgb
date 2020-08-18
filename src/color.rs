use core::fmt;

/// The location for applying a color
pub enum Canvas {
    Background,
    Foreground,
}

/// Generates ANSI escape sequences for a specific kind of color
pub trait FormatColor {
    /// Apply the color
    fn prelude(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result;

    /// Undo the color application
    #[allow(unused)]
    fn epilogue(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result {
        f.write_str("\x1B[0m")
    }
}

/// Something with a background color
pub struct WithBackground<T, TFormatColor: FormatColor> {
    t: T,
    color: TFormatColor,
}

/// Something with a foreground color
pub struct WithForeground<T, TFormatColor: FormatColor> {
    t: T,
    color: TFormatColor,
}

/// Adds a foreground or background color
pub trait Colorable<TFormatColor: FormatColor>: Sized {
    /// Adds the given background color
    fn bg(self, color: TFormatColor) -> WithBackground<Self, TFormatColor>;

    /// Adds the given foreground color
    fn fg(self, color: TFormatColor) -> WithForeground<Self, TFormatColor>;
}

impl<T, TFormatColor: FormatColor> Colorable<TFormatColor> for T {
    fn bg(self, color: TFormatColor) -> WithBackground<Self, TFormatColor> {
        WithBackground { t: self, color }
    }

    fn fg(self, color: TFormatColor) -> WithForeground<Self, TFormatColor> {
        WithForeground { t: self, color }
    }
}

macro_rules! impl_me {
    ($bound:path) => {
        impl<T: $bound, TFormatColor: FormatColor> $bound for WithBackground<T, TFormatColor> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.color
                    .prelude(f, Canvas::Background)
                    .and_then(|_| self.t.fmt(f))
                    .and_then(|_| self.color.epilogue(f, Canvas::Background))
            }
        }

        impl<T: $bound, TFormatColor: FormatColor> $bound for WithForeground<T, TFormatColor> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.color
                    .prelude(f, Canvas::Foreground)
                    .and_then(|_| self.t.fmt(f))
                    .and_then(|_| self.color.epilogue(f, Canvas::Foreground))
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
