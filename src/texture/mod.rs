use std::sync::Arc;

use crate::math::{Vec2, Vec3};

mod perlin;

pub use perlin::Noise;

pub trait Texture: Send + Sync {
    fn value(&self, uv: Vec2, p: &Vec3) -> Vec3;
}

pub trait TexPtr {
    fn into(self) -> Arc<dyn Texture>;
}

impl<T> TexPtr for T
where
    T: Texture + 'static,
{
    fn into(self) -> Arc<dyn Texture> {
        Arc::new(self)
    }
}

impl<T> TexPtr for Arc<T>
where
    T: Texture + 'static,
{
    fn into(self) -> Arc<dyn Texture> {
        self
    }
}

impl TexPtr for Arc<dyn Texture> {
    fn into(self) -> Arc<dyn Texture> {
        self
    }
}

pub struct Constant {
    color: Vec3,
}

impl Constant {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for Constant {
    fn value(&self, _uv: Vec2, _p: &Vec3) -> Vec3 {
        self.color
    }
}
