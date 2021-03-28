use std::{f32::consts::PI, sync::Arc};

use nalgebra_glm::{vec3, Vec3};

use crate::hit::{
    aabb::{surrounding_box, Aabb},
    hittable::{HitRecord, Hittable, UV},
    ray::Ray,
};
use crate::material::Material;

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

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        hit_sphere(
            self.radius,
            self.center,
            self.material.as_ref(),
            r,
            t_min,
            t_max,
        )
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - vec3(self.radius, self.radius, self.radius),
            self.center + vec3(self.radius, self.radius, self.radius),
        ))
    }
}

pub fn hit_sphere<'a>(
    radius: f32,
    center: Vec3,
    material: &'a dyn Material,
    r: &Ray,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord<'a>> {
    let oc = r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let half_b = oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return None;
    }
    let sqrtd = discriminant.sqrt();
    let root = (-half_b - sqrtd) / a;
    let root = if root < t_min || t_max < root {
        let root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }
        root
    } else {
        root
    };

    let t = root;
    let p = r.point_at_parameter(t);
    let outward_normal: Vec3 = (p - center) / radius;

    Some(HitRecord::new(
        t,
        p,
        outward_normal,
        material,
        get_sphere_uv(outward_normal),
    ))
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn boxed(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Box<Self> {
        Self::new(center0, center1, time0, time1, radius, material).into()
    }

    fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        hit_sphere(
            self.radius,
            self.center(r.time()),
            self.material.as_ref(),
            r,
            t_min,
            t_max,
        )
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center0 - vec3(self.radius, self.radius, self.radius),
            self.center0 + vec3(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center1 - vec3(self.radius, self.radius, self.radius),
            self.center1 + vec3(self.radius, self.radius, self.radius),
        );
        Some(surrounding_box(&box0, &box1))
    }
}

fn get_sphere_uv(p: Vec3) -> UV {
    // let phi = f32::atan2(p.z, p.x);
    // let theta = f32::asin(p.y);
    // let u = 1.0 - (phi + PI) / (2.0 * PI);
    // let v = (theta + FRAC_PI_2) / PI;
    // UV { u, v }

    let theta = f32::acos(-p.y);
    let phi = f32::atan2(-p.z, p.x) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;
    UV { u, v }
}
