use std::f64::consts::PI;

use rand::random;

use crate::{
    hit::Hitable,
    math::{dot, random_cosine_direction, Onb, Vec3},
};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: Onb::build_from(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = dot(&direction.normalize(), self.uvw.w());
        if cosine <= 0. {
            0.
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(&random_cosine_direction())
    }
}

pub struct HitablePdf<T> {
    o: Vec3,
    ptr: T,
}

impl<T> HitablePdf<T> {
    pub fn new(o: Vec3, ptr: T) -> Self {
        Self { o, ptr }
    }
}

impl<T> Pdf for HitablePdf<T>
where
    T: Hitable,
{
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

impl Pdf for HitablePdf<&dyn Hitable> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct MixturePdf<P0, P1> {
    p0: P0,
    p1: P1,
}

impl<P0, P1> MixturePdf<P0, P1> {
    pub fn new(p1: P0, p2: P1) -> Self {
        Self { p0: p1, p1: p2 }
    }
}

impl<P0, P1> Pdf for MixturePdf<P0, P1>
where
    P0: Pdf,
    P1: Pdf,
{
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random::<f64>() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
