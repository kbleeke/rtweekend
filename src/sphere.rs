use std::{
    ops::{Bound::*, RangeBounds},
    sync::Arc,
};

use crate::{
    material::Material,
    ray::{HitRecord, Hitable, Ray},
    Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn boxed(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Box<Self> {
        Self::new(center, radius, material).into()
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if (Excluded(t_min), Excluded(t_max)).contains(&temp) {
                let p = r.point_at_parameter(temp);
                let rec = HitRecord::new(
                    temp,
                    r.point_at_parameter(temp),
                    (p - self.center) / self.radius,
                    self.material.as_ref(),
                );
                return Some(rec);
            }
            temp = (-b + discriminant.sqrt()) / a;
            if (Excluded(t_min), Excluded(t_max)).contains(&temp) {
                let p = r.point_at_parameter(temp);
                let rec = HitRecord::new(
                    temp,
                    r.point_at_parameter(temp),
                    (p - self.center) / self.radius,
                    self.material.as_ref(),
                );
                return Some(rec);
            }
        }
        None
    }
}
