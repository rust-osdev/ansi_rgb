use crate::Canvas;
use core::fmt;

/// Generates ANSI escape sequences for a specific kind of color
pub trait Color {
    /// Apply the color
    fn prelude(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result;

    /// Undo the color application
    #[allow(unused)]
    fn epilogue(&self, f: &mut fmt::Formatter, canvas: Canvas) -> fmt::Result {
        f.write_str("\x1B[0m")
    }
}
