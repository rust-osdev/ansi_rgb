use crate::colors::*;
use core::fmt;

pub trait Background: Sized {
    fn background(self, rgb: Rgb) -> WithBackground<Self>;
}

pub struct WithBackground<T> {
    t: T,
    rgb: Rgb
}

impl<T> Background for &T {
    fn background(self, rgb: Rgb) -> WithBackground<Self> {
        WithBackground {
            t: self,
            rgb
        }
    }
}

impl<T> fmt::Display for WithBackground<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[48;2;{};{};{}m{}\x1B[0m", self.rgb.r, self.rgb.g, self.rgb.b, self.t)
    }
}

impl<T> fmt::Debug for WithBackground<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[48;2;{};{};{}m{:?}\x1B[0m", self.rgb.r, self.rgb.g, self.rgb.b, self.t)
    }
}