use std::sync::Arc;

use crate::math::{Vec2, Vec3};

use super::{HitRecord, Ray};

#[derive(Debug, Clone, Copy, Default)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
    pub pdf: f64,
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