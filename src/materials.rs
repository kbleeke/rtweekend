use std::{f64::consts::PI, sync::Arc};

use rand::random;

use crate::{
    hit::{HitRecord, Material, Ray, Scatter},
    math::{
        dot, random_cosine_direction, random_in_unit_sphere, reflect, refract, vec3, Onb, Vec2,
        Vec3,
    },
    texture::{self, TexPtr, Texture},
};

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: impl TexPtr) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }

    #[allow(unused)]
    pub fn constant(albedo: Vec3) -> Self {
        Self {
            albedo: TexPtr::into(texture::Constant::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        // let mut target = rec.p + random_in_hemisphere(rec.normal);
        // let mut target = rec.normal + random_in_unit_sphere().normalize();
        // let target = random_in_hemisphere(rec.normal);

        // if target.near_zero() {
        //     target = rec.normal;
        // }

        let uvw = Onb::build_from(&rec.normal);
        let direction = uvw.local(&random_cosine_direction());

        let scattered = Ray::new(rec.p, direction.normalize());
        Some(Scatter {
            attenuation: self.albedo.value(rec.uv, &rec.p),
            scattered,
            // pdf: dot(rec.normal, scattered.direction()) / PI,
            // pdf: 0.5 / PI,
            pdf: dot(uvw.w(), scattered.direction()) / PI,
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(&rec.normal, &scattered.direction().normalize());
        if cosine < 0. {
            0.
        } else {
            cosine / PI
        }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&r_in.direction().normalize(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        if dot(scattered.direction(), &rec.normal) > 0. {
            Some(Scatter {
                scattered,
                attenuation: self.albedo,
                pdf: 0.0,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * f64::powi(1. - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(r_in.direction(), &rec.normal);
        let attenuation = vec3(1., 1., 1.);

        let (outward_normal, ni_over_nt, cosine) = if dot(r_in.direction(), &rec.normal) > 0. {
            (
                -rec.normal,
                self.ref_idx,
                self.ref_idx * dot(r_in.direction(), &rec.normal) / r_in.direction().length(),
            )
        } else {
            (
                rec.normal,
                1. / self.ref_idx,
                -dot(r_in.direction(), &rec.normal) / r_in.direction().length(),
            )
        };

        let v = refract(r_in.direction(), &outward_normal, ni_over_nt)
            .filter(|_| {
                let reflect_prob = Dielectric::schlick(cosine, self.ref_idx);
                random::<f64>() >= reflect_prob
            })
            .unwrap_or(reflected);
        Some(Scatter {
            scattered: Ray::new(rec.p, v),
            attenuation,
            pdf: 0.0,
        })
    }
}

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: impl TexPtr) -> Self {
        Self {
            texture: texture.into(),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Scatter> {
        None
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, uv: Vec2, p: &Vec3) -> Vec3 {
        if rec.front_face {
            self.texture.value(uv, p)
        } else {
            Vec3::zero()
        }
    }
}
