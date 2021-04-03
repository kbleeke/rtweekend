use nalgebra as na;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    e: na::Vector2<f64>,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            e: na::Vector2::new(x, y),
        }
    }

    pub fn new1(x: f64) -> Self {
        Self::new(x, x)
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn u(&self) -> f64 {
        self.e[0]
    }

    pub fn v(&self) -> f64 {
        self.e[1]
    }
}

impl Vec2 {
    pub fn length(&self) -> f64 {
        self.e.magnitude()
    }

    pub fn length_squared(&self) -> f64 {
        self.e.magnitude_squared()
    }

    pub fn normalize(&self) -> Vec2 {
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
}

pub fn vec2(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

// mod linalg {
//     use super::Vec2;

//     #[inline(always)]
//     pub fn dot(v1: Vec2, v2: Vec2) -> f64 {
//         v1.e.dot(&v2.e)
//     }

//     #[inline(always)]
//     pub fn reflect(v: Vec2, n: Vec2) -> Vec2 {
//         v - 2. * dot(v, n) * n
//     }

//     #[inline(always)]
//     pub fn cross(v1: Vec2, v2: Vec2) -> Vec2 {
//         Vec2 {
//             e: v1.e.cross(&v2.e),
//         }
//     }

//     pub fn unit_vector(v: Vec2) -> Vec2 {
//         v.normalize()
//     }

//     pub fn refract(v: Vec2, n: Vec2, ni_over_nt: f64) -> Option<Vec2> {
//         let uv = unit_vector(v);
//         let dt = dot(uv, n);
//         let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
//         if discriminant > 0. {
//             Some(ni_over_nt * (uv - n * dt) - n * f64::sqrt(discriminant))
//         } else {
//             None
//         }
//     }
// }
// pub use linalg::*;

mod r {
    use rand::{thread_rng, Rng};

    use super::{vec2, Vec2};

    pub fn random_in_unit_sphere() -> Vec2 {
        let mut rng = thread_rng();
        loop {
            let p: Vec2 = 2. * vec2(rng.gen(), rng.gen()) - 1.;
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
        ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    use super::Vec2;

    impl Neg for Vec2
    where
        f64: Neg<Output = f64> + Copy,
    {
        type Output = Vec2;

        fn neg(self) -> Self::Output {
            Self { e: -self.e }
        }
    }

    impl Index<usize> for Vec2 {
        type Output = f64;

        fn index(&self, index: usize) -> &Self::Output {
            &self.e[index]
        }
    }

    macro_rules! op {
        ($traitname:ty, $f: ident, $o:tt) => {
            impl $traitname for Vec2 {
                type Output = Vec2;

                fn $f(self, rhs:Vec2) -> Self::Output {
                    Self::new(self.e.x $o rhs.e.x,
                              self.e.y $o rhs.e.y)
                }
            }
        };
    }

    macro_rules! op_assign {
        ($traitname:ty, $f: ident, $o:tt) => {
            impl $traitname for Vec2 {
                fn $f(&mut self, rhs:Vec2) {
                    self.e.x $o rhs.e.x;
                    self.e.y $o rhs.e.y;
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
            impl $trt1 for Vec2 {
                type Output = Vec2;

                fn $f(self, rhs: f64) -> Self::Output {
                    self $o super::vec2(rhs, rhs)
                }
            }

            impl $trt2 for f64 {
                type Output = Vec2;

                fn $f(self, rhs: Vec2) -> Self::Output {
                    super::vec2(self, self) $o rhs
                }
            }
        };
    }

    op_scalar!(Add<f64>, Add<Vec2>, add, +);
    op_scalar!(Sub<f64>, Sub<Vec2>, sub, -);
    op_scalar!(Mul<f64>, Mul<Vec2>, mul, *);
    op_scalar!(Div<f64>, Div<Vec2>, div, /);

    impl MulAssign<f64> for Vec2 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = *self * rhs
        }
    }

    impl Sum for Vec2 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Vec2::zero(), |f, i| f + i)
        }
    }
}

impl Default for Vec2
where
    f64: Default,
{
    fn default() -> Self {
        Self {
            e: Default::default(),
        }
    }
}
