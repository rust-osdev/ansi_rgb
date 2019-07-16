use core::fmt;

pub trait Blink: Sized {
    fn blink(self) -> WithBlink<Self>;
}

pub struct WithBlink<T> {
    t: T
}

impl<T> Blink for &T {
    fn blink(self) -> WithBlink<Self> {
        WithBlink {
            t: self
        }
    }
}

impl<T> fmt::Display for WithBlink<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[5m{}\x1B[0m", self.t)
    }
}

impl<T> fmt::Debug for WithBlink<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[5m{:?}\x1B[0m", self.t)
    }
}