pub struct Vector3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
}

pub struct Vector2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}