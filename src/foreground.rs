use crate::colors::*;
use core::fmt;

pub trait Foreground: Sized {
    fn foreground(self, rgb: Rgb) -> WithForeground<Self>;
}

pub struct WithForeground<T> {
    t: T,
    rgb: Rgb
}

impl<T> Foreground for &T {
    fn foreground(self, rgb: Rgb) -> WithForeground<Self> {
        WithForeground {
            t: self,
            rgb
        }
    }
}

impl<T> fmt::Display for WithForeground<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[38;2;{};{};{}m{}\x1B[0m", self.rgb.r, self.rgb.g, self.rgb.b, self.t)
    }
}

impl<T> fmt::Debug for WithForeground<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[38;2;{};{};{}m{:?}\x1B[0m", self.rgb.r, self.rgb.g, self.rgb.b, self.t)
    }
}