use std::sync::Arc;

use nalgebra_glm::Vec3;

use crate::shapes::rect::*;
use crate::{
    hit::hittable::Hittable,
    hit::{
        aabb::Aabb,
        hittable::{flip_normals, HitRecord},
        ray::Ray,
    },
    material::Material,
};

pub struct Cuboid {
    pmin: Vec3,
    pmax: Vec3,
    faces: [Box<dyn Hittable>; 6],
}

impl Cuboid {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Self {
        let faces = [
            XyRect::boxed(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone()),
            flip_normals(XyRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone())),
            XzRect::boxed(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone()),
            flip_normals(XzRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone())),
            YzRect::boxed(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone()),
            flip_normals(YzRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone())),
        ];

        Self {
            pmin: p0,
            pmax: p1,
            faces,
        }
    }

    pub fn boxed(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Box<Self> {
        Box::new(Self::new(p0, p1, material))
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        if Aabb::new(self.pmin, self.pmax).hit(r, t_min, t_max) {
            self.faces.hit(r, t_min, t_max)
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::hit::aabb::Aabb> {
        Some(Aabb::new(self.pmin, self.pmax))
    }
}
