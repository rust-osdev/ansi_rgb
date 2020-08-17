use crate::Canvas;
use core::fmt;

pub trait Color {
    fn prelude(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result;
    fn epilogue(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result;
}
