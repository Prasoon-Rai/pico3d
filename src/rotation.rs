use libm::{cosf, sinf};
pub struct RotateOnAxis;

#[allow(non_snake_case)]
impl RotateOnAxis {
    pub fn X(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
        let c = cosf(angle);
        let s = sinf(angle);

        let new_y = y * c - z * s;
        let new_z = y * s + z * c;

        (x, new_y, new_z)
    }

    pub fn Y(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
        let c = cosf(angle);
        let s = sinf(angle);

        let new_x = x * c - z * s;
        let new_z = x * s + z * c;

        (new_x, y, new_z)
    }

    pub fn Z(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
        let c = cosf(angle);
        let s = sinf(angle);

        let new_x = x * c - y * s;
        let new_y = x * s + y * c;

        (new_x, new_y, z)
    }
}