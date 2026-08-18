use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable},
};
use crate::render::Viewport;

pub fn draw_point<D>(
    viewport: &Viewport,
    display: &mut D,
    x: f32,
    y: f32,
    z: f32,
    size: f32,
    style: &PrimitiveStyle<D::Color>,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    let (sx, sy) = viewport.project(x, y, z);

    Rectangle::new(
        Point::new((sx - size / 2.0) as i32, (sy - size / 2.0) as i32),
        Size::new(size as u32, size as u32),
    )
        .draw_styled(style, display)
}