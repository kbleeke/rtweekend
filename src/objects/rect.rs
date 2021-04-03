use std::{marker::PhantomData, sync::Arc};

use rand::{thread_rng, Rng};

use crate::{
    hit::{Aabb, HitRecord, Hitable, MatPtr, Material, Ray},
    math::{dot, vec2, vec3, Vec2, Vec3},
};

pub trait Plane: Send + Sync {
    fn a() -> usize;
    fn b() -> usize;
    fn k() -> usize;

    fn permute(a: f64, b: f64, k: f64) -> Vec3;
}

pub struct XY;
impl Plane for XY {
    fn a() -> usize {
        0
    }

    fn b() -> usize {
        1
    }

    fn k() -> usize {
        2
    }

    fn permute(a: f64, b: f64, k: f64) -> Vec3 {
        vec3(a, b, k)
    }
}

pub struct XZ;
impl Plane for XZ {
    fn a() -> usize {
        0
    }

    fn b() -> usize {
        2
    }

    fn k() -> usize {
        1
    }

    fn permute(a: f64, b: f64, k: f64) -> Vec3 {
        vec3(a, k, b)
    }
}

pub struct YZ;
impl Plane for YZ {
    fn a() -> usize {
        1
    }

    fn b() -> usize {
        2
    }

    fn k() -> usize {
        0
    }

    fn permute(a: f64, b: f64, k: f64) -> Vec3 {
        vec3(k, a, b)
    }
}
pub struct Rect<T> {
    a: Vec2,
    b: Vec2,
    k: f64,
    mat: Arc<dyn Material>,
    _plane: PhantomData<fn(T)>,
}

impl<T> Rect<T> {
    pub fn new(a: Vec2, b: Vec2, k: f64, mat: impl MatPtr) -> Self {
        Self {
            a,
            b,
            k,
            mat: mat.into(),
            _plane: PhantomData,
        }
    }
}

impl<T> Hitable for Rect<T>
where
    T: Plane,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin()[T::k()]) / r.direction()[T::k()];
        if t < t_min || t_max < t {
            return None;
        }
        let a = r.origin()[T::a()] + t * r.direction()[T::a()];
        let b = r.origin()[T::b()] + t * r.direction()[T::b()];

        if a < self.a[0] || a > self.a[1] || b < self.b[0] || b > self.b[1] {
            return None;
        }

        let uv = vec2(
            (a - self.a[0]) / (self.a[1] - self.a[0]),
            (b - self.b[0]) / (self.b[1] - self.b[0]),
        );

        Some(HitRecord::new(
            r,
            t,
            r.at(t),
            T::permute(0.0, 0.0, 1.0),
            uv,
            self.mat.as_ref(),
        ))
    }

    fn bounding_box(&self) -> crate::hit::Aabb {
        Aabb::new(
            T::permute(self.a[0], self.b[0], self.k - 0.0001),
            T::permute(self.a[1], self.b[1], self.k + 0.0001),
        )
    }

    fn pdf_value(&self, origin: &Vec3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(*origin, *v), 0.001, f64::INFINITY) {
            let area = (self.a[1] - self.a[0]) * (self.b[1] - self.b[0]);
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = dot(&v, &rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let mut rng = thread_rng();
        let random_point = T::permute(
            rng.gen_range(self.a[0]..=self.a[1]),
            rng.gen_range(self.b[0]..=self.b[1]),
            self.k,
        );
        random_point - o
    }
}

pub type XyRect = Rect<XY>;
pub type XzRect = Rect<XZ>;
pub type YzRect = Rect<YZ>;

impl<T> Clone for Rect<T> {
    fn clone(&self) -> Self {
        Self::new(self.a, self.b, self.k, self.mat.clone())
    }
}
