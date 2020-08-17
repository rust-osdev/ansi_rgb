use crate::Canvas;
use crate::Color;
use core::fmt;

/// Adds a background color
pub trait Background<TColor: Color>: Sized {
    /// Adds the given background color
    fn bg(self, color: TColor) -> WithBackground<Self, TColor>;
}

/// Something with a background color
pub struct WithBackground<T, TColor: Color> {
    t: T,
    color: TColor,
}

impl<T, TColor: Color> Background<TColor> for T {
    fn bg(self, color: TColor) -> WithBackground<Self, TColor> {
        WithBackground { t: self, color }
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
