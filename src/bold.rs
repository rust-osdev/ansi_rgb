use core::fmt;

pub trait Bold<T>: Sized {
    fn bold(self) -> WithBold<Self>;
}

pub struct WithBold<T> {
    t: T
}

impl<T> Bold<T> for &T {
    fn bold(self) -> WithBold<Self> {
        WithBold {
            t: self
        }
    }
}

impl<T> fmt::Display for WithBold<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[1m{}\x1B[0m", self.t)
    }
}

impl<T> fmt::Debug for WithBold<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[1m{:?}\x1B[0m", self.t)
    }
}