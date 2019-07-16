use core::fmt;

pub trait Italic: Sized {
    fn italic(self) -> WithItalic<Self>;
}

pub struct WithItalic<T> {
    t: T
}

impl<T> Italic for &T {
    fn italic(self) -> WithItalic<Self> {
        WithItalic {
            t: self
        }
    }
}

impl<T> fmt::Display for WithItalic<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[3m{}\x1B[0m", self.t)
    }
}

impl<T> fmt::Debug for WithItalic<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[3m{:?}\x1B[0m", self.t)
    }
}