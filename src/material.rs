use std::sync::Arc;

use cgmath::{vec3, InnerSpace};
use rand::random;

use crate::{ray::*, Vec3};

pub struct Scattered {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<Scattered>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Arc<Self> {
        Self { albedo }.into()
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &mut HitRecord) -> Option<Scattered> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some(Scattered {
            scattered: Ray::new(rec.p, target - rec.p),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Arc<Self> {
        let fuzz = f.min(1.0);
        Self { albedo, fuzz }.into()
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<Scattered> {
        let reflected = reflect(r_in.direction().normalize(), rec.normal);
        let scattered = Scattered {
            scattered: Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere()),
            attenuation: self.albedo,
        };
        if scattered.scattered.direction().dot(rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0_f32 * random::<Vec3>() - vec3(1.0, 1.0, 1.0);
        if p.magnitude2() >= 1.0 {
            break p;
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Arc<Self> {
        Self { ref_idx }.into()
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<Scattered> {
        let reflected = reflect(r_in.direction(), rec.normal);
        let attenuation = vec3(1.0, 1.0, 1.0);

        let outward_normal;
        let ni_over_nt;
        let cosine;
        if r_in.direction().dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine =self.ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().magnitude();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction().dot(rec.normal) / r_in.direction().magnitude();
        }

        match refract(r_in.direction(), outward_normal, ni_over_nt) {
            Some(refracted) if random::<f32>() >= schlick(cosine, self.ref_idx) => Scattered {
                scattered: Ray::new(rec.p, refracted),
                attenuation,
            }
            .into(),
            _ => Scattered {
                scattered: Ray::new(rec.p, reflected),
                attenuation,
            }
            .into(),
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
