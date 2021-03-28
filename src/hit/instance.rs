use std::f32::consts::PI;

use nalgebra_glm::{mat2x2, vec1, vec3, Mat2x2, Vec2, Vec3};

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Translate<T> {
    offset: Vec3,
    hittable: T,
}

pub fn translate<T>(h: T, offset: Vec3) -> Translate<T> {
    Translate {
        offset,
        hittable: h,
    }
}

impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        self.hittable.hit(&moved_ray, t_min, t_max).map(|mut rec| {
            rec.p += self.offset;
            rec
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<super::aabb::Aabb> {
        self.hittable
            .bounding_box(t0, t1)
            .map(|bb| Aabb::new(bb.min() + self.offset, bb.max() + self.offset))
    }
}

pub fn rotate_y<T>(h: T, degrees: f32) -> RotateY<T>
where
    T: Hittable,
{
    RotateY::new(h, degrees)
}

pub struct RotateY<T> {
    ptr: T,
    rotate: Mat2x2,
    bbox: Option<Aabb>,
}

impl<T> RotateY<T>
where
    T: Hittable,
{
    pub fn new(hittable: T, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let rotate = mat2x2(cos_theta, -sin_theta, sin_theta, cos_theta);

        let bbox = hittable
            .bounding_box(0.0, 1.0)
            .map(|bbox| map_bbox(bbox, rotate));

        Self {
            ptr: hittable,
            rotate,
            bbox,
        }
    }
}

fn map_bbox(bbox: Aabb, rotate: Mat2x2) -> Aabb {
    let mut max: Vec3 = Vec3::repeat(f32::MAX);
    let mut min: Vec3 = Vec3::repeat(f32::MIN);
    let one: Vec3 = Vec3::repeat(1.);
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ijk: Vec3 = vec3(i, j, k).cast();
                let xyz: Vec3 =
                    ijk.component_mul(&bbox.max()) + (one - ijk).component_mul(&bbox.min());

                let xz: Vec2 = rotate * xyz.xz();
                let tester: Vec3 = vec3(xz[0], xyz.y, xz[1]);

                for c in 0..3 {
                    if tester[i] > max[i] {
                        max[c] = tester[c]
                    }
                    if tester[i] < min[i] {
                        min[i] = tester[i]
                    }
                }
            }
        }
    }
    Aabb::new(min, max)
}

impl<H> Hittable for RotateY<H>
where
    H: Hittable,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let origin: Vec2 = self.rotate * r.origin().xz();
        let direction: Vec2 = self.rotate * r.direction().xz();

        let rotated_ray = Ray::new(
            vec3(origin[0], r.origin().y, origin[1]),
            vec3(direction[0], r.direction().y, direction[1]),
            r.time(),
        );
        self.ptr.hit(&rotated_ray, t_min, t_max).map(|mut rec| {
            let p = self.rotate * rec.p.xz();
            let normal = self.rotate * rec.normal.xz();

            rec.p.x = p[0];
            rec.p.z = p[1];
            rec.normal.x = normal[0];
            rec.normal.z = normal[1];
            rec
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        self.bbox.clone()
    }
}
