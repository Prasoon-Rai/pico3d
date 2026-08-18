# pico3d

A tiny software 3-D renderer for microcontrollers. It rotates points, divides
them by `z` for perspective, maps the result onto a screen, and draws it through
[`embedded-graphics`](https://docs.rs/embedded-graphics).

Setting the display up is yours — `Viewport` never owns it, it just borrows a
`DrawTarget` when you ask it to draw. So the same code works on a real panel or
against a simulator on your laptop.

## Using it

```toml
[dependencies]
pico3d = { path = "../path/to/pico3d" }
```

```rust
use pico3d::{RotateOnAxis, Viewport};

// however you set your display up
let viewport = Viewport::from(display.size());

for &(x, y, z) in &points {
    let (rx, ry, rz) = RotateOnAxis::Y(x, y, z, angle);
    viewport.draw_point(&mut display, rx, ry, rz + 1.0, 2.0, &style)?;
}
```

`rz + 1.0` pushes the point away from the camera. `z` is distance from the
camera and has to be positive — at zero, `project` divides by zero.

## What's here

| item | what it does |
|------|--------------|
| `RotateOnAxis::X/Y/Z` | rotates a point around one axis by an angle in radians |
| `Viewport::to_screen` | maps `-1.0 ..= 1.0` onto pixels, origin in the middle, `y` pointing up |
| `Viewport::project` | perspective divide, then `to_screen` |
| `Viewport::draw_point` | draws a point as a filled square of a fixed pixel size |

That's the whole crate — the primitives to build on, nothing above them.

## Testing

`no_std`, but with no hardware dependencies, so it builds and tests on the host:

```bash
cargo test
```
