use crate::Canvas;
use core::fmt;

pub trait Color {
    fn prelude(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result;
    fn epilogue(&self, f: &mut core::fmt::Formatter, _canvas: crate::Canvas) -> core::fmt::Result {
        f.write_str("\x1B[0m")
    }
}
