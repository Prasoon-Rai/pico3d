use core::cmp;
use embedded_graphics::{
    prelude::*,
};
use crate::math::Vector3;

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
        let half = cmp::min(self.width, self.height) as f32 / 2.0;
        (
            // Scaled according to display:
            // (x + 1.0) / 2.0 * self.width as f32,
            // (1.0 - (y + 1.0) / 2.0) * self.height as f32,

            // Uniformly scaled:
            (self.width / 2) as f32 + x * half,
            (self.height / 2) as f32 - y * half
        )
    }

    pub fn project(&self, cords: &Vector3) -> (f32, f32) {
        self.to_screen(cords.x / cords.z, cords.y / cords.z)
    }
}

impl From<Size> for Viewport {
    fn from(size: Size) -> Self {
        Self::new(size.width, size.height)
    }
}