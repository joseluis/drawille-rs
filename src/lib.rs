//! `drawille` – a terminal graphics library for Rust, based on the Python library
//! [drawille](https://github.com/asciimoo/drawille).
//!
//! This crate provides an interface for utilising Braille characters to draw a picture to a
//! terminal, allowing for much smaller pixels but losing proper colour support.
//!
//! # Example
//!
//! ```
//! extern crate drawille;
//!
//! use drawille::Canvas;
//!
//! fn main() {
//!     let mut canvas = Canvas::new(10, 10);
//!     canvas.set(5, 4);
//!     canvas.line(2, 2, 8, 8);
//!     assert_eq!(canvas.frame(), [
//! " ⢄    ",
//! "  ⠙⢄  ",
//! "    ⠁ "].join("\n"));
//! }
//! ```

extern crate fnv;

mod canvas;
mod turtle;

pub use canvas::Canvas;
pub use turtle::Turtle;


