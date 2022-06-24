#![no_std]
#![doc = include_str!("../README.md")]

mod color;
#[cfg(feature = "default")]
mod colors;

pub use color::*;
#[cfg(feature = "default")]
pub use colors::*;
