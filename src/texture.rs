use std::sync::Arc;

use image::RgbImage;
use nalgebra_glm::vec3;
use nalgebra_glm::Vec3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Arc<Self> {
        Arc::new(Self { color })
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    event: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, event: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { odd, event })
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.event.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> Arc<Self> {
        Arc::new(Self { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vec3) -> Vec3 {
        let i = (u * self.image.width() as f32) as u32;
        let i = i.clamp(0, self.image.width() - 1);

        let j = ((1.0 - v) * self.image.height() as f32 - 0.001) as u32;
        let j = j.clamp(0, self.image.height() - 1);

        let rgb = self.image.get_pixel(i, j);
        vec3(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        )
    }
}
