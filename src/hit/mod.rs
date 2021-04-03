use std::sync::Arc;

use crate::math::{dot, vec3, Vec2, Vec3};

mod aabb;
pub use aabb::surrounding_box;
pub use aabb::Aabb;

mod list;
pub use list::*;

mod bvh;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { a, b }
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}

pub struct HitRecord<'m> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'m dyn Material,
    pub uv: Vec2,
}

impl<'m> HitRecord<'m> {
    pub fn new(
        r: &Ray,
        t: f64,
        p: Vec3,
        outward_normal: Vec3,
        uv: Vec2,
        material: &'m dyn Material,
    ) -> Self {
        let front_face = dot(r.direction(), &outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            t,
            p,
            normal,
            front_face,
            material,
            uv,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, normal: Vec3) {
        let front_face = dot(r.direction(), &normal) < 0.;
        let normal = if front_face { normal } else { -normal };
        self.normal = normal;
        self.front_face = front_face;
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let _ = (o, v);
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let _ = o;
        vec3(1, 0, 0)
    }
}

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
