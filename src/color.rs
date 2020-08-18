use core::fmt;

/// The location for applying a color
pub enum Canvas {
    Background,
    Foreground,
}

/// Generates ANSI escape sequences for a specific kind of color
pub trait Color {
    /// Apply the color
    fn prelude(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result;

    /// Undo the color application
    #[allow(unused)]
    fn epilogue(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result {
        f.write_str("\x1B[0m")
    }
}

/// Something with a background color
pub struct WithBackground<T, TColor: Color> {
    t: T,
    color: TColor,
}

/// Something with a foreground color
pub struct WithForeground<T, TColor: Color> {
    t: T,
    color: TColor,
}

/// Adds a foreground or background color
pub trait Colorable<TColor: Color>: Sized {
    /// Adds the given background color
    fn bg(self, color: TColor) -> WithBackground<Self, TColor>;

    /// Adds the given foreground color
    fn fg(self, color: TColor) -> WithForeground<Self, TColor>;
}

impl<T, TColor: Color> Colorable<TColor> for T {
    fn bg(self, color: TColor) -> WithBackground<Self, TColor> {
        WithBackground { t: self, color }
    }

    fn fg(self, color: TColor) -> WithForeground<Self, TColor> {
        WithForeground { t: self, color }
    }
}

macro_rules! impl_me {
    ($bound:path, $format_arg:expr) => {
        impl<T: $bound, TColor: Color> $bound for WithBackground<T, TColor> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.color
                    .prelude(f, Canvas::Background)
                    .and_then(|_| write!(f, $format_arg, self.t))
                    .and_then(|_| self.color.epilogue(f, Canvas::Background))
            }
        }

        impl<T: $bound, TColor: Color> $bound for WithForeground<T, TColor> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.color
                    .prelude(f, Canvas::Foreground)
                    .and_then(|_| write!(f, $format_arg, self.t))
                    .and_then(|_| self.color.epilogue(f, Canvas::Foreground))
            }
        }
    };
}

impl_me!(fmt::Binary, "{:b}");
impl_me!(fmt::Debug, "{:?}");
impl_me!(fmt::Display, "{}");
impl_me!(fmt::LowerExp, "{:e}");
impl_me!(fmt::LowerHex, "{:x}");
impl_me!(fmt::Octal, "{:o}");
impl_me!(fmt::Pointer, "{:p}");
impl_me!(fmt::UpperExp, "{:E}");
impl_me!(fmt::UpperHex, "{:X}");
