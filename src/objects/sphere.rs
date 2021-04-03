use std::{f64::consts::PI, sync::Arc};

use rand::random;

use crate::{
    hit::MatPtr,
    hit::{Aabb, HitRecord, Hitable, Material, Ray},
    math::{dot, vec2, vec3, Onb, Vec2, Vec3},
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
        let half_b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
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

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if self.hit(&Ray::new(*o, *v), 0.001, f64::INFINITY).is_some() {
            let cos_theta_max =
                f64::sqrt(1. - self.radius * self.radius / (self.center - o).length_squared());
            let solid_angle = 2. * PI * (1. - cos_theta_max);
            1. / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw = Onb::build_from(&direction);

        uvw.local(&random_to_sphere(&self.radius, &distance_squared))
    }
}

fn random_to_sphere(radius: &f64, distance_squared: &f64) -> Vec3 {
    let r1: f64 = random();
    let r2: f64 = random();
    let z = 1. + r2 * (f64::sqrt(1. - radius * radius / distance_squared) - 1.);

    let phi = 2. * PI * r1;
    let (sin, cos) = phi.sin_cos();
    let sq = f64::sqrt(1. - z * z);

    vec3(cos * sq, sin * sq, z)
}

fn get_sphere_uv(p: Vec3) -> Vec2 {
    let theta = f64::acos(-p.y());
    let phi = f64::atan2(-p.z(), p.x()) + PI;
    vec2(phi / (2. * PI), theta / PI)
}
