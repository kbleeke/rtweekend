use std::{f64::consts::PI, sync::Arc};

use rand::random;

use crate::{
    hit::{HitRecord, Material, Ray, Scatter},
    math::{dot, random_in_unit_sphere, reflect, refract, vec3, Vec2, Vec3},
    pdf::CosinePdf,
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
        let attenuation = self.albedo.value(rec.uv, &rec.p);
        let pdf = CosinePdf::new(&rec.normal);
        Some(Scatter::new_diffuse(attenuation, Box::new(pdf)))
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
        Some(Scatter::new_specular(scattered, self.albedo))
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * f64::powi(1. - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = vec3(1, 1, 1);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::schlick(cos_theta, refraction_ratio) > random() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        Some(Scatter::new_specular(
            Ray::new(rec.p, direction),
            attenuation,
        ))
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
