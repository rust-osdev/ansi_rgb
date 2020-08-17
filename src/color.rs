use crate::Canvas;
use core::fmt;

pub trait Color {
    fn prelude(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result;
    #[allow(unused)]
    fn epilogue(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result {
        f.write_str("\x1B[0m")
    }
}
