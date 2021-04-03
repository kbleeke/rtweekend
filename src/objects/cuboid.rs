use crate::objects::rect::{XyRect, XzRect, YzRect};
use crate::{
    hit::{Aabb, HitRecord, Hitable, MatPtr, Ray},
    math::{vec2, Vec3},
    transform::HitableExt,
};

pub struct Cuboid {
    pmin: Vec3,
    pmax: Vec3,
    faces: [Box<dyn Hitable>; 6],
}

impl Cuboid {
    pub fn new(p0: Vec3, p1: Vec3, material: impl MatPtr) -> Self {
        let material = material.into();

        let faces = [
            XyRect::new(
                vec2(p0.x(), p1.x()),
                vec2(p0.y(), p1.y()),
                p1.z(),
                material.clone(),
            )
            .boxed(),
            XyRect::new(
                vec2(p0.x(), p1.x()),
                vec2(p0.y(), p1.y()),
                p0.z(),
                material.clone(),
            )
            .flip_normals()
            .boxed(),
            XzRect::new(
                vec2(p0.x(), p1.x()),
                vec2(p0.z(), p1.z()),
                p1.y(),
                material.clone(),
            )
            .boxed(),
            XzRect::new(
                vec2(p0.x(), p1.x()),
                vec2(p0.z(), p1.z()),
                p0.y(),
                material.clone(),
            )
            .flip_normals()
            .boxed(),
            YzRect::new(
                vec2(p0.y(), p1.y()),
                vec2(p0.z(), p1.z()),
                p1.x(),
                material.clone(),
            )
            .boxed(),
            YzRect::new(
                vec2(p0.y(), p1.y()),
                vec2(p0.z(), p1.z()),
                p0.x(),
                material.clone(),
            )
            .flip_normals()
            .boxed(),
        ];

        Self {
            pmin: p0,
            pmax: p1,
            faces,
        }
    }
}

impl Hitable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(self.pmin, self.pmax)
    }
}
