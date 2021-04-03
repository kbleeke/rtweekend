use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    hit::{surrounding_box, Aabb, HitRecord, Hitable, Ray},
    math::{vec3, Vec3},
};

impl<T> Hitable for Vec<T>
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_slice().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Aabb {
        self.as_slice().bounding_box()
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        self.as_slice().pdf_value(o, v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.as_slice().random(o)
    }
}

impl<T, const N: usize> Hitable for [T; N]
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Aabb {
        self.as_ref().bounding_box()
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        self.as_ref().pdf_value(o, v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.as_ref().random(o)
    }
}

impl<T> Hitable for [T]
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        let mut hit_anything = None;

        for hitable in self {
            if let Some(hit_rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t;
                hit_anything = Some(hit_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        assert!(!self.is_empty(), "hitable list must not be empty");

        let mut it = self.iter();
        let first = it.next().unwrap().bounding_box();
        it.fold(first, |b, h| surrounding_box(&b, &h.bounding_box()))
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let weight = 1. / self.len() as f64;
        self.iter()
            .map(|object| weight * object.pdf_value(o, v))
            .sum()
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.choose(&mut thread_rng())
            .map(|obj| obj.random(o))
            .unwrap_or(vec3(1, 0, 0))
    }
}
