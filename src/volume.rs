use std::sync::Arc;

use rand::random;

use crate::{
    hit::{HitRecord, Hitable, Material, Ray, Scatter},
    math::{random_in_unit_sphere, vec3, Vec2},
    texture::{TexPtr, Texture},
};

pub struct ConstantMedium {
    phase_function: Isotropic,
    density: f64,
    boundary: Box<dyn Hitable>,
}

impl ConstantMedium {
    pub fn new(density: f64, boundary: Box<dyn Hitable>, texture: impl TexPtr) -> Self {
        Self {
            phase_function: Isotropic::new(texture.into()),
            density,
            boundary,
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let db = random::<f64>() < 0.00001;
        if let Some(mut rec1) = self.boundary.hit(r, f64::MIN, f64::MAX) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f64::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0. {
                    rec1.t = 0.;
                }

                let distance_inside_boundary = (rec2.t - rec1.t) * r.direction().length();
                let hit_distance = -(1. / self.density) * random::<f64>().ln();
                if hit_distance < distance_inside_boundary {
                    let t = rec1.t + hit_distance / r.direction().length();
                    let p = r.at(t);
                    return Some(HitRecord::new(
                        r,
                        t,
                        p,
                        vec3(1., 0., 0.),
                        Vec2::zero(),
                        &self.phase_function,
                    ));
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> crate::hit::Aabb {
        self.boundary.bounding_box()
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        Some(Scatter {
            scattered: Ray::new(rec.p, random_in_unit_sphere()),
            attenuation: self.albedo.value(rec.uv, &rec.p),
            pdf: 0.0,
        })
    }
}
