use std::sync::Arc;

use itertools::iproduct;

use crate::{
    hit::{Aabb, HitRecord, Hitable, Ray},
    math::{vec3, Vec3},
};

pub struct FlipNormals<T>
where
    T: ?Sized,
{
    inner: T,
}

impl<T> Hitable for FlipNormals<T>
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.inner.hit(r, t_min, t_max).map(|mut rec| {
            rec.normal = -rec.normal;
            rec
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.inner.bounding_box()
    }
}

pub fn flip_normals<T>(inner: T) -> FlipNormals<T>
where
    T: Hitable,
{
    FlipNormals { inner }
}

pub trait HitableExt {
    fn flip_normals(self) -> FlipNormals<Self>;
    fn boxed(self) -> Box<dyn Hitable>;
    fn translate(self, offset: Vec3) -> Translate<Self>;
    fn rotate_y(self, angle: f64) -> RotateY<Self>;
    fn flip_face(self) -> FlipFace<Self>;
    fn shared(self) -> Arc<dyn Hitable>;
}

impl<T> HitableExt for T
where
    T: Hitable + 'static,
{
    fn flip_normals(self) -> FlipNormals<Self> {
        FlipNormals { inner: self }
    }

    fn boxed(self) -> Box<dyn Hitable> {
        Box::new(self)
    }

    fn shared(self) -> Arc<dyn Hitable> {
        Arc::new(self)
    }

    fn translate(self, offset: Vec3) -> Translate<Self> {
        Translate {
            offset,
            inner: self,
        }
    }

    fn rotate_y(self, angle: f64) -> RotateY<Self> {
        RotateY::new(angle, self)
    }

    fn flip_face(self) -> FlipFace<Self> {
        FlipFace { ptr: self }
    }
}

pub struct Translate<T>
where
    T: ?Sized,
{
    offset: Vec3,
    inner: T,
}

impl<T> Hitable for Translate<T>
where
    T: Hitable + 'static,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, *r.direction());
        self.inner.hit(&moved_r, t_min, t_max).map(|mut rec| {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            rec
        })
    }

    fn bounding_box(&self) -> Aabb {
        let bbox = self.inner.bounding_box();
        Aabb::new(bbox.min() + self.offset, bbox.max() + self.offset)
    }
}

pub struct RotateY<T>
where
    T: ?Sized,
{
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
    inner: T,
}

impl<T> RotateY<T>
where
    T: Hitable + 'static,
{
    fn new(angle: f64, inner: T) -> Self {
        let radians = angle.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = inner.bounding_box();

        let mut min = vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = vec3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        iproduct!(0..2u8, 0..2u8, 0..2u8).for_each(|(i, j, k)| {
            let ijk = vec3(i, j, k);
            let xyz = ijk * bbox.max() + (1. - ijk) * bbox.min();

            let newx = cos_theta * xyz.x() + sin_theta * xyz.z();
            let newz = -sin_theta * xyz.x() + cos_theta * xyz.z();

            let tester = vec3(newx, xyz.y(), newz);

            for c in 0..3 {
                min[c] = min[c].min(tester[c]);
                max[c] = max[c].max(tester[c]);
            }
        });

        let bbox = Aabb::new(min, max);

        Self {
            sin_theta,
            cos_theta,
            bbox,
            inner,
        }
    }
}

impl<T> Hitable for RotateY<T>
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = *r.origin();
        let mut direction = *r.direction();

        origin[0] = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin[2] = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        direction[0] = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        direction[2] = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let rotated_ray = Ray::new(origin, direction);
        self.inner.hit(&rotated_ray, t_min, t_max).map(|mut rec| {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
            p[2] = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

            normal[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
            normal[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

            rec.p = p;
            rec.set_face_normal(&rotated_ray, normal);

            rec
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct FlipFace<T>
where
    T: ?Sized,
{
    ptr: T,
}

impl<T> Hitable for FlipFace<T>
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.ptr.hit(r, t_min, t_max).map(|mut rec| {
            rec.front_face = !rec.front_face;
            rec
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.ptr.bounding_box()
    }
}
