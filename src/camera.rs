use std::f64::consts::PI;

use crate::{
    hit::Ray,
    math::{cross, unit_vector, vec3, Vec3},
};

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let theta = vfov * PI / 180.;
        let half_height = f64::tan(theta / 2.);
        let half_width = aspect * half_height;

        let origin = lookfrom;
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2. * half_width * u;
        let vertical = 2. * half_height * v;

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    #[allow(unused)]
    pub fn aspect_fov(vfov: f64, aspect: f64) -> Self {
        let theta = vfov * PI / 180.;
        let half_height = f64::tan(theta / 2.);
        let half_width = aspect * half_height;

        let lower_left_corner = vec3(-half_width, -half_height, -1.);
        let horizontal = vec3(2. * half_width, 0., 0.);
        let vertical = vec3(0., 2. * half_height, 0.);
        let origin = Vec3::zero();

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
