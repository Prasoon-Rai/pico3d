use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    pub fn to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x + 1.0) / 2.0 * self.width as f32,
            (1.0 - (y + 1.0) / 2.0) * self.height as f32,
        )
    }

    pub fn project(&self, x: f32, y: f32, z: f32) -> (f32, f32) {
        self.to_screen(x / z, y / z)
    }
}

impl From<Size> for Viewport {
    fn from(size: Size) -> Self {
        Self::new(size.width, size.height)
    }
}