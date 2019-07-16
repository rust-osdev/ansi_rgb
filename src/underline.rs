use core::fmt;

pub trait Underline: Sized {
    fn underline(self) -> WithUnderline<Self>;
}

pub struct WithUnderline<T> {
    t: T
}

impl<T> Underline for &T {
    fn underline(self) -> WithUnderline<Self> {
        WithUnderline {
            t: self
        }
    }
}

impl<T> fmt::Display for WithUnderline<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[3m{}\x1B[0m", self.t)
    }
}

impl<T> fmt::Debug for WithUnderline<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[3m{:?}\x1B[0m", self.t)
    }
}