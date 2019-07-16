use crate::colors::*;
use core::fmt;

pub trait Foreground: Sized {
    fn fg(self, rgb: Rgb) -> WithForeground<Self>;
}

pub struct WithForeground<T> {
    t: T,
    rgb: Rgb
}

impl<T> Foreground for &T {
    fn fg(self, rgb: Rgb) -> WithForeground<Self> {
        WithForeground {
            t: self,
            rgb
        }
    }
}

macro_rules! impl_me {
    ($bound:path, $format_arg:expr) => {
        impl<T> $bound for WithForeground<T>
        where T: $bound {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!("\x1B[38;2;{};{};{}m", $format_arg, "\x1B[0m"), self.rgb.r, self.rgb.g, self.rgb.b, self.t)
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