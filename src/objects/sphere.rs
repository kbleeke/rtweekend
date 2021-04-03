use std::{f64::consts::PI, sync::Arc};

use crate::{
    hit::MatPtr,
    hit::{Aabb, HitRecord, Hitable, Material, Ray},
    math::{dot, vec2, Vec2, Vec3},
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: impl MatPtr) -> Self {
        Self {
            center,
            radius,
            material: material.into(),
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let half_b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        return Some(HitRecord::new(
            r,
            root,
            p,
            outward_normal,
            get_sphere_uv(outward_normal),
            self.material.as_ref(),
        ));
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(self.center - self.radius, self.center + self.radius)
    }
}

fn get_sphere_uv(p: Vec3) -> Vec2 {
    // let phi = f64::atan2(p.z(), p.x());
    // let theta = f64::asin(p.y());
    // vec2(1. - (phi + PI) / (2. * PI), (theta + FRAC_PI_2) / PI)

    let theta = f64::acos(-p.y());
    let phi = f64::atan2(-p.z(), p.x()) + PI;
    vec2(phi / (2. * PI), theta / PI)
}
