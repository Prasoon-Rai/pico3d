//! The 3-D bits of a tiny software renderer for microcontrollers.
//!
//! This crate is deliberately small: it rotates points, divides them by `z` for
//! perspective, maps the result onto a screen, and draws it through
//! [`embedded_graphics`]. Setting up the display is left to you — hand
//! [`Viewport`] any `DrawTarget` and it will draw into it.

#![cfg_attr(not(test), no_std)]

pub mod render;
pub mod rotation;
pub mod core;

pub use render::Viewport;
pub use rotation::RotateOnAxis;
pub use core::DrawPoint;
pub use core::DrawLine;
pub use core::DrawCubeWire;