use cgmath::Vector3;

use crate::{material::Material, Vec3};
pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>,
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        Self { a, b }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.a
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        self.a + t * self.b
    }
}
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a dyn Material) -> Self {
        Self {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;
}
