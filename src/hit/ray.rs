use crate::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, time: f32) -> Self {
        Self { a, b, time }
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }
}
