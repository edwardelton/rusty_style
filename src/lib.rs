//! # Rusty Style
//!
//! Rusty Style is a library for styling text in the terminal.
//! It is inspired by the library lipgloss in golang.

pub mod color;
pub mod style;

pub use color::Color;

pub use style::Style;
