use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{
    hit::{Aabb, HitRecord, Hitable, MatPtr, Material, Ray},
    math::{dot, vec2, vec3, Vec2, Vec3},
};

macro_rules! rect {
    ($name:ident, $dim1:ident, $dim2:ident, $kdim:ident, $boxf:ident ,$normalf:ident) => {
        pub struct $name {
            $dim1: Vec2,
            $dim2: Vec2,
            k: f64,
            material: Arc<dyn Material>,
        }

        impl $name {
            pub fn new($dim1: Vec2, $dim2: Vec2, k: f64, material: impl MatPtr) -> Self {
                Self {
                    $dim1,
                    $dim2,
                    k,
                    material: material.into(),
                }
            }
        }

        impl Hitable for $name {
            fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                let t = (self.k - r.origin().$kdim()) / r.direction().$kdim();
                if t < t_min || t_max < t {
                    return None;
                }
                let $dim1 = r.origin().$dim1() + t * r.direction().$dim1();
                let $dim2 = r.origin().$dim2() + t * r.direction().$dim2();

                if $dim1 < self.$dim1[0]
                    || $dim1 > self.$dim1[1]
                    || $dim2 < self.$dim2[0]
                    || $dim2 > self.$dim2[1]
                {
                    return None;
                }

                let uv = vec2(
                    ($dim1 - self.$dim1[0]) / (self.$dim1[1] - self.$dim1[0]),
                    ($dim2 - self.$dim2[0]) / (self.$dim2[1] - self.$dim2[0]),
                );

                Some(HitRecord::new(
                    r,
                    t,
                    r.at(t),
                    $normalf(),
                    uv,
                    self.material.as_ref(),
                ))
            }

            fn bounding_box(&self) -> crate::hit::Aabb {
                $boxf(self.$dim1, self.$dim2, self.k)
            }
        }
    };
}

rect!(XyRect, x, y, z, xy_box, xy_normal);
rect!(YzRect, y, z, x, yz_box, yz_normal);

fn xy_box(a: Vec2, b: Vec2, k: f64) -> Aabb {
    Aabb::new(vec3(a[0], b[0], k - 0.0001), vec3(a[1], b[1], k + 0.0001))
}

fn xz_box(a: Vec2, b: Vec2, k: f64) -> Aabb {
    Aabb::new(vec3(a[0], k - 0.0001, b[0]), vec3(a[1], k + 0.0001, b[1]))
}

fn yz_box(a: Vec2, b: Vec2, k: f64) -> Aabb {
    Aabb::new(vec3(k - 0.0001, a[0], b[0]), vec3(k + 0.0001, a[1], b[1]))
}

fn xy_normal() -> Vec3 {
    vec3(0., 0., 1.)
}
fn xz_normal() -> Vec3 {
    vec3(0., 1., 0.)
}
fn yz_normal() -> Vec3 {
    vec3(1., 0., 0.)
}

#[derive(Clone)]
pub struct XzRect {
    x: Vec2,
    z: Vec2,
    k: f64,
    material: Arc<dyn Material>,
}

impl XzRect {
    pub fn new(x: Vec2, z: Vec2, k: f64, material: impl MatPtr) -> Self {
        Self {
            x,
            z,
            k,
            material: material.into(),
        }
    }
}

impl Hitable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t_max < t {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();

        if x < self.x[0] || x > self.x[1] || z < self.z[0] || z > self.z[1] {
            return None;
        }

        let uv = vec2(
            (x - self.x[0]) / (self.x[1] - self.x[0]),
            (z - self.z[0]) / (self.z[1] - self.z[0]),
        );

        Some(HitRecord::new(
            r,
            t,
            r.at(t),
            xz_normal(),
            uv,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self) -> crate::hit::Aabb {
        xz_box(self.x, self.z, self.k)
    }

    fn pdf_value(&self, origin: &Vec3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(*origin, *v), 0.001, f64::INFINITY) {
            let area = (self.x[1] - self.x[0]) * (self.z[1] - self.z[0]);
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = dot(&v, &rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let mut rng = thread_rng();
        let random_point = vec3(
            rng.gen_range(self.x[0]..=self.x[1]),
            self.k,
            rng.gen_range(self.z[0]..=self.z[1]),
        );
        random_point - o
    }
}
