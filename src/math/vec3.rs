use nalgebra as na;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: na::Vector3<f64>,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            e: na::Vector3::new(x, y, z),
        }
    }

    pub fn new1(x: f64) -> Self {
        Self::new(x, x, x)
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        self.e.magnitude()
    }

    pub fn length_squared(&self) -> f64 {
        self.e.magnitude_squared()
    }

    pub fn normalize(&self) -> Vec3 {
        Self {
            e: self.e.normalize(),
        }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn product(&self) -> f64 {
        self[0] * self[1] * self[2]
    }

    pub fn map(&self, f: impl FnMut(f64) -> f64) -> Self {
        Self { e: self.e.map(f) }
    }

    pub fn near_zero(&self) -> bool {
        self.x().abs() < f64::EPSILON
            && self.y().abs() < f64::EPSILON
            && self.z().abs() < f64::EPSILON
    }
}

pub fn vec3<F1, F2, F3>(x: F1, y: F2, z: F3) -> Vec3
where
    F1: Into<f64>,
    F2: Into<f64>,
    F3: Into<f64>,
{
    Vec3::new(x.into(), y.into(), z.into())
}

mod linalg {
    use super::Vec3;

    #[inline(always)]
    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.e.dot(&v2.e)
    }

    #[inline(always)]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2. * dot(v, n) * n
    }

    #[inline(always)]
    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            e: v1.e.cross(&v2.e),
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v.normalize()
    }

    pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
        let uv = unit_vector(v);
        let dt = dot(uv, n);
        let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
        if discriminant > 0. {
            Some(ni_over_nt * (uv - n * dt) - n * f64::sqrt(discriminant))
        } else {
            None
        }
    }
}
pub use linalg::*;

mod r {
    use rand::{thread_rng, Rng};

    use super::Vec3;

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let p: Vec3 = 2. * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - 1.;
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }
}
pub use r::*;

mod ops {
    use std::{
        iter::Sum,
        ops::{
            Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
        },
    };

    use super::Vec3;

    impl Neg for Vec3
    where
        f64: Neg<Output = f64> + Copy,
    {
        type Output = Vec3;

        fn neg(self) -> Self::Output {
            Self { e: -self.e }
        }
    }

    impl Index<usize> for Vec3 {
        type Output = f64;

        fn index(&self, index: usize) -> &Self::Output {
            &self.e[index]
        }
    }

    impl IndexMut<usize> for Vec3 {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.e[index]
        }
    }

    macro_rules! op {
        ($traitname:ty, $f: ident, $o:tt) => {
            impl $traitname for Vec3 {
                type Output = Vec3;

                fn $f(self, rhs:Vec3) -> Self::Output {
                    Self::new(self.e.x $o rhs.e.x,
                              self.e.y $o rhs.e.y,
                              self.e.z $o rhs.e.z)
                }
            }
        };
    }

    macro_rules! op_assign {
        ($traitname:ty, $f: ident, $o:tt) => {
            impl $traitname for Vec3 {
                fn $f(&mut self, rhs:Vec3) {
                    self.e.x $o rhs.e.x;
                    self.e.y $o rhs.e.y;
                    self.e.z $o rhs.e.z;
                }
            }
        };
    }

    op!(Add, add, +);
    op!(Sub, sub, -);
    op!(Mul, mul, *);
    op!(Div, div, /);

    op_assign!(AddAssign, add_assign, +=);
    op_assign!(SubAssign, sub_assign, -=);
    op_assign!(MulAssign, mul_assign, *=);
    op_assign!(DivAssign, div_assign, /=);

    macro_rules! op_scalar {
        ($trt1:ty, $trt2:ty, $f: ident, $o:tt) => {
            impl $trt1 for Vec3 {
                type Output = Vec3;

                fn $f(self, rhs: f64) -> Self::Output {
                    self $o super::vec3(rhs, rhs, rhs)
                }
            }

            impl $trt2 for f64 {
                type Output = Vec3;

                fn $f(self, rhs: Vec3) -> Self::Output {
                    super::vec3(self, self, self) $o rhs
                }
            }
        };
    }

    op_scalar!(Add<f64>, Add<Vec3>, add, +);
    op_scalar!(Sub<f64>, Sub<Vec3>, sub, -);
    op_scalar!(Mul<f64>, Mul<Vec3>, mul, *);
    op_scalar!(Div<f64>, Div<Vec3>, div, /);

    impl MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = *self * rhs
        }
    }

    impl Sum for Vec3 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Vec3::zero(), |f, i| f + i)
        }
    }
}

impl Default for Vec3
where
    f64: Default,
{
    fn default() -> Self {
        Self {
            e: Default::default(),
        }
    }
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0. {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
