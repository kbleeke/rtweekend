use crate::math::{vec3, Vec3};

use super::Ray;
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        (0..3).all(|a| {
            let inv_d = 1.0 / r.direction()[a];
            let (t0, t1) = {
                let t0 = (self.min[a] - r.origin()[a]) * inv_d;
                let t1 = (self.max[a] - r.origin()[a]) * inv_d;
                if inv_d < 0.0 {
                    (t1, t0)
                } else {
                    (t0, t1)
                }
            };
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            t_max > t_min
        })
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = vec3(
        fmin(box0.min.x(), box1.min.x()),
        fmin(box0.min.y(), box1.min.y()),
        fmin(box0.min.z(), box1.min.z()),
    );
    let big = vec3(
        fmax(box0.max.x(), box1.max.x()),
        fmax(box0.max.y(), box1.max.y()),
        fmax(box0.max.z(), box1.max.z()),
    );
    Aabb::new(small, big)
}

fn fmin(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}

fn fmax(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}
