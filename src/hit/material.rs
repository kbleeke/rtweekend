use std::sync::Arc;

use crate::math::{Vec2, Vec3};

use super::{HitRecord, Ray};

pub enum ScatterKind {
    Diffuse { pdf: Box<dyn Pdf> },
    Specular { specular_ray: Ray },
}
pub struct Scatter {
    attenuation: Vec3,
    kind: ScatterKind,
}

impl Scatter {
    pub fn new_diffuse(attenuation: Vec3, pdf: Box<dyn Pdf>) -> Self {
        Self {
            attenuation,
            kind: ScatterKind::Diffuse { pdf },
        }
    }

    pub fn new_specular(ray: Ray, attenuation: Vec3) -> Self {
        Self {
            attenuation,
            kind: ScatterKind::Specular { specular_ray: ray },
        }
    }

    pub fn attenuation(&self) -> &Vec3 {
        &self.attenuation
    }

    pub fn kind(&self) -> &ScatterKind {
        &self.kind
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let _ = (r_in, rec, scattered);
        0.0
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, uv: Vec2, p: &Vec3) -> Vec3 {
        let _ = (r_in, rec, uv, p);
        Vec3::zero()
    }
}

pub trait MatPtr {
    fn into(self) -> Arc<dyn Material>;
}

impl<T> MatPtr for T
where
    T: Material + 'static,
{
    fn into(self) -> Arc<dyn Material> {
        Arc::new(self)
    }
}

impl<T> MatPtr for Arc<T>
where
    T: Material + 'static,
{
    fn into(self) -> Arc<dyn Material> {
        self
    }
}

impl MatPtr for Arc<dyn Material> {
    fn into(self) -> Arc<dyn Material> {
        self
    }
}

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

impl Pdf for &dyn Pdf {
    fn value(&self, direction: &Vec3) -> f64 {
        (*self).value(direction)
    }

    fn generate(&self) -> Vec3 {
        (*self).generate()
    }
}
