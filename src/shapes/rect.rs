use std::sync::Arc;

use nalgebra_glm::{vec2, vec3, Vec2};

use crate::{
    hit::{
        aabb::Aabb,
        hittable::{HitRecord, Hittable, UV},
        ray::Ray,
    },
    material::Material,
};

pub struct XyRect {
    material: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XyRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }

    pub fn boxed(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Box<Self> {
        Box::new(Self::new(x0, x1, y0, y1, k, material))
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - r.origin().z) / r.direction().z;
        if t < t_min || t > t_max {
            return None;
        }

        let xy: Vec2 = r.origin().xy() + t * r.direction().xy();
        if xy.x < self.x0 || xy.x > self.x1 || xy.y < self.y0 || xy.y > self.y1 {
            return None;
        }

        let uv: Vec2 = (xy - vec2(self.x0, self.y0))
            .component_div(&(vec2(self.x1, self.y1) - vec2(self.x0, self.y0)));

        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            vec3(0.0, 0.0, 1.0),
            self.material.as_ref(),
            UV { u: uv[0], v: uv[1] },
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::hit::aabb::Aabb> {
        Some(Aabb::new(
            vec3(self.x0, self.y0, self.k - 0.0001),
            vec3(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XzRect {
    material: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }

    pub fn boxed(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Box<Self> {
        Box::new(Self::new(x0, x1, z0, z1, k, material))
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - r.origin().y) / r.direction().y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().x + t * r.direction().x;
        let z = r.origin().z + t * r.direction().z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            vec3(0.0, 1.0, 0.0),
            self.material.as_ref(),
            UV { u, v },
        ))

        // let xz: Vec2 = r.origin().xz() + t * r.direction().xz();
        // if xz.x < self.x0 || xz.x > self.x1 || xz[1] < self.z0 || xz[1] > self.z1 {
        //     return None;
        // }

        // let uv: Vec2 = (xz - vec2(self.x0, self.z0))
        //     .component_div(&(vec2(self.x1, self.z1) - vec2(self.x0, self.z0)));

        // Some(HitRecord::new(
        //     t,
        //     r.point_at_parameter(t),
        //     vec3(0.0, 1.0, 0.0),
        //     self.material.as_ref(),
        //     UV { u: uv[0], v: uv[1] },
        // ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::hit::aabb::Aabb> {
        Some(Aabb::new(
            vec3(self.x0, self.k - 0.0001, self.z0),
            vec3(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct YzRect {
    material: Arc<dyn Material>,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }

    pub fn boxed(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Box<Self> {
        Box::new(Self::new(y0, y1, z0, z1, k, material))
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - r.origin().x) / r.direction().x;
        if t < t_min || t > t_max {
            return None;
        }

        let yz: Vec2 = r.origin().yz() + t * r.direction().yz();
        if yz[0] < self.y0 || yz[0] > self.y1 || yz[1] < self.z0 || yz[1] > self.z1 {
            return None;
        }

        let uv: Vec2 = (yz - vec2(self.y0, self.z0))
            .component_div(&(vec2(self.y1, self.z1) - vec2(self.y0, self.z0)));

        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            vec3(1.0, 0.0, 0.0),
            self.material.as_ref(),
            UV { u: uv[0], v: uv[1] },
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::hit::aabb::Aabb> {
        Some(Aabb::new(
            vec3(self.k - 0.0001, self.y0, self.z0),
            vec3(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
