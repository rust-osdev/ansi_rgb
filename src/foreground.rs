use crate::Canvas;
use crate::Color;
use core::fmt;

/// Adds a foreground color
pub trait Foreground<TColor: Color>: Sized {
    /// Adds the given foreground color
    fn fg(self, color: TColor) -> WithForeground<Self, TColor>;
}

/// Something with a foreground color
pub struct WithForeground<T, TColor: Color> {
    t: T,
    color: TColor,
}

impl<T, TColor: Color> Foreground<TColor> for T {
    fn fg(self, color: TColor) -> WithForeground<Self, TColor> {
        WithForeground { t: self, color }
    }
}

macro_rules! impl_me {
    ($bound:path, $format_arg:expr) => {
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
