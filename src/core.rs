/// Basic geometric 3D shapes drawing functions

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable},
};

use embedded_graphics::primitives::{Line,};
use libm::{cosf, sinf};
use crate::math::Vector3;
use crate::render::Viewport;
use core::f32::consts::PI;

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

pub fn DrawPointV<D>(
    viewport: &Viewport,
    display: &mut D,
    position: Vector3,
    size: f32,
    style: &PrimitiveStyle<D::Color>,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    let (sx, sy) = viewport.project(position.x, position.y, position.z);

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

pub fn DrawLineV<D>(
    viewport: &Viewport,
    display: &mut D,
    start: &Vector3,
    end: &Vector3,
    style: &PrimitiveStyle<D::Color>,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    let (xS, yS) = viewport.project(start.x, start.y, start.z);
    let (xE, yE) = viewport.project(end.x, end.y, end.z);

    Line::new(Point::new(xS as i32, yS as i32), Point::new(xE as i32, yE as i32)).draw_styled(style, display)
}

pub fn DrawCubeWire<D> (
    viewport: &Viewport,
    display: &mut D,
    position: Vector3,
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

    let wn = width * 0.5 + position.x;
    let hn = height * 0.5 + position.y;
    let dn = depth * 0.5 + position.z;

    let cube_vertices = [
        // Front face:
        (wn, hn, dn),
        (-wn, hn, dn),
        (wn, -hn, dn),
        (-wn, -hn, dn),

        // Back face:
        (wn, hn, -dn),
        (-wn, hn, -dn),
        (wn, -hn, -dn),
        (-wn, -hn, -dn),
    ];

    for &(a, b) in &CUBE_EDGES {
        DrawLine(viewport, display, cube_vertices[a], cube_vertices[b], style)?;
    }

    Ok(())
}

pub fn DrawCubeWireV<D> (
    viewport: &Viewport,
    display: &mut D,
    position: Vector3,
    dimensions: Vector3,
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

    let wn = dimensions.x * 0.5 + position.x;
    let hn = dimensions.y * 0.5 + position.y;
    let dn = dimensions.z * 0.5 + position.z;

    let cube_vertices = [
        // Front face:
        (wn, hn, dn),
        (-wn, hn, dn),
        (wn, -hn, dn),
        (-wn, -hn, dn),

        // Back face:
        (wn, hn, -dn),
        (-wn, hn, -dn),
        (wn, -hn, -dn),
        (-wn, -hn, -dn),
    ];

    for &(a, b) in &CUBE_EDGES {
        DrawLine(viewport, display, cube_vertices[a], cube_vertices[b], style)?;
    }

    Ok(())
}


pub fn DrawSphereWire<D> (
    viewport: &Viewport,
    display: &mut D,
    centerPos: Vector3,
    radius: f32,
    rings: usize,
    slices: usize,
    style: &PrimitiveStyle<D::Color>
) -> Result<(), D::Error>
where
    D: DrawTarget
{
    let spherePoint = |radius: f32, i: usize, j: usize, rings: usize, slices: usize| -> Vector3 {
        let phi = PI * (i as f32 / rings as f32);
        let theta = 2.0 * PI * (j as f32 / slices as f32);
        Vector3 {
            x: centerPos.x + radius * sinf(phi) * cosf(theta),
            y: centerPos.y + radius * cosf(phi),
            z: centerPos.z + radius * sinf(phi) * sinf(theta),
        }
    };

    for i in 0..=rings {
        for j in 0..=slices {
            let current = spherePoint(radius, i, j, rings, slices);

            if j < slices {
                let right = spherePoint(radius, i, j + 1, rings, slices);
                DrawLineV(viewport, display, &current, &right, style)?
            }

            if i < rings {
                let bottom = spherePoint(radius, i + 1, j, rings, slices);
                DrawLineV(viewport, display, &current, &bottom, style)?
            }
        }
    }

    Ok(())
}

