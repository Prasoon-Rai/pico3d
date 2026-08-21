/// Basic geometric 3D shapes drawing functions

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable},
};
use embedded_graphics::pixelcolor::{Rgb666, RgbColor};
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyleBuilder};
use crate::render::Viewport;

pub fn DrawPoint<D>(
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
        Point::new((sx - size / 2.0) as i32, (sy - size / 2.0) as i32), Size::new(size as u32, size as u32), )
        .draw_styled(style, display)
}

pub fn DrawLine<D>(
    viewport: &Viewport,
    display: &mut D,
    start: (f32, f32, f32),
    end: (f32, f32, f32),
    style: &PrimitiveStyle<D::Color>,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    let (xS, yS) = viewport.project(start.0, start.1, start.2);
    let (xE, yE) = viewport.project(end.0, end.1, end.2);

    Line::new(Point::new(xS as i32, yS as i32), Point::new(xE as i32, yE as i32)).draw_styled(style, display)
}

pub fn DrawCubeWire<D> (
    viewport: &Viewport,
    display: &mut D,
    width: f32,
    height: f32,
    depth: f32,
    style: &PrimitiveStyle<D::Color>,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    const CUBE_EDGES: [(usize, usize); 12] = [
        (0, 1), (1, 3), (3, 2), (2, 0),  // front face
        (4, 5), (5, 7), (7, 6), (6, 4),  // back face 
        (0, 4), (1, 5), (2, 6), (3, 7),  // connectors
    ];

    let cube_vertices = [
        // Front face:
        (width * 0.5, height * 0.5, depth * 0.5),
        (-width * 0.5, height * 0.5, depth * 0.5),
        (width * 0.5, -height * 0.5, depth * 0.5),
        (-width * 0.5, -height * 0.5, depth * 0.5),

        // Back face:
        (width * 0.5, height * 0.5, -depth * 0.5),
        (-width * 0.5, height * 0.5, -depth * 0.5),
        (width * 0.5, -height * 0.5, -depth * 0.5),
        (-width * 0.5, -height * 0.5, -depth * 0.5),
    ];

    for &(a, b) in &CUBE_EDGES {
        DrawLine(viewport, display, cube_vertices[a], cube_vertices[b], style)?;
    }

    Ok(())
}