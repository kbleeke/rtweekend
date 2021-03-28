use nalgebra_glm::Vec3;

use super::aabb::surrounding_box;
use super::{aabb::Aabb, ray::Ray};

use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct UV {
    pub u: f32,
    pub v: f32,
}

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub uv: UV,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a dyn Material, uv: UV) -> Self {
        Self {
            t,
            p,
            normal,
            material,
            uv,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        Self { list }
    }

    pub fn into_inner(self) -> Vec<Box<dyn Hittable>> {
        self.list
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for hitable in &self.list {
            if let Some(temp_rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        let mut it = self.list.iter();
        let first = it.next().and_then(|h| h.bounding_box(t0, t1))?;
        it.try_fold(first, |b, h| {
            h.bounding_box(t0, t1).map(|b2| surrounding_box(&b, &b2))
        })
    }
}

impl<'a> Hittable for &[Box<dyn Hittable>] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for hitable in self.iter() {
            if let Some(temp_rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        let mut it = self.iter();
        let first = it.next().and_then(|h| h.bounding_box(t0, t1))?;
        it.try_fold(first, |b, h| {
            h.bounding_box(t0, t1).map(|b2| surrounding_box(&b, &b2))
        })
    }
}

impl<const N: usize> Hittable for [Box<dyn Hittable>; N] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for hitable in self.iter() {
            if let Some(temp_rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        let mut it = self.iter();
        let first = it.next().and_then(|h| h.bounding_box(t0, t1))?;
        it.try_fold(first, |b, h| {
            h.bounding_box(t0, t1).map(|b2| surrounding_box(&b, &b2))
        })
    }
}

pub struct FlipNormals<T> {
    hittable: T,
}

impl<T> FlipNormals<T> {
    pub fn new(hittable: T) -> Self {
        Self { hittable }
    }
}

pub fn flip_normals<T: Hittable + 'static>(hittable: T) -> Box<dyn Hittable> {
    Box::new(FlipNormals::new(hittable))
}

impl<T> Hittable for FlipNormals<T>
where
    T: Hittable,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.hittable.hit(r, t_min, t_max).map(|mut rec| {
            rec.normal = -rec.normal;
            rec
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.hittable.bounding_box(t0, t1)
    }
}

impl Hittable for FlipNormals<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.hittable.hit(r, t_min, t_max).map(|mut rec| {
            rec.normal = -rec.normal;
            rec
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.hittable.bounding_box(t0, t1)
    }
}
