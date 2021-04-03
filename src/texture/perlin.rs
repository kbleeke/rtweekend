use itertools::iproduct;
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};

use crate::math::{dot, unit_vector, vec3, Vec2, Vec3};

use super::Texture;

struct Perlin {
    // ranfloat: Box<[f64]>,
    ranvec: Box<[Vec3]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    fn noise(&self, p: &Vec3) -> f64 {
        let floored = p.map(f64::floor);
        let uvw = *p - floored;

        let i = floored.x() as i32;
        let j = floored.y() as i32;
        let k = floored.z() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = Default::default();
        iproduct!(0..2, 0..2, 0..2).for_each(|(di, dj, dk)| {
            c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                ^ self.perm_y[((j + dj as i32) & 255) as usize]
                ^ self.perm_z[((k + dk as i32) & 255) as usize]];
        });

        perlin_interp(c, uvw)
    }

    fn turb(&self, p: &Vec3, depth: usize) -> f64 {
        let mut p = *p;
        let mut accum = 0.;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.;
        }
        accum.abs()
    }
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], uvw: Vec3) -> f64 {
    let uuvvww = uvw * uvw * (3. - 2. * uvw);

    iproduct!(0..2, 0..2, 0..2)
        .map(|(i, j, k)| {
            let c = c[i][j][k];
            (vec3(i as f64, j as f64, k as f64), c)
        })
        .map(|(ijk, c)| {
            let weight_v = uvw - ijk;
            (ijk * uuvvww + (1. - ijk) * (1. - uuvvww)).product() * dot(c, weight_v)
        })
        .sum()
}

#[allow(unused)]
fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    iproduct!(0..2, 0..2, 0..2)
        .map(|(i, j, k)| {
            let c = c[i][j][k];
            (i as f64, j as f64, k as f64, c)
        })
        .map(|(i, j, k, c)| {
            c * (i * u + (1. - i) * (1. - u))
                * (j * v + (1. - j) * (1. - v))
                * (k * w + (1. - k) * (1. - w))
        })
        .sum()
}

// fn perlin_generate() -> Box<[f64]> {
//     (0..256).map(|_| random()).collect()
// }

fn perlin_generate() -> Box<[Vec3]> {
    let mut rng = thread_rng();

    (0..256)
        .map(|_| unit_vector(-1. + 2. * vec3(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())))
        .collect()
}

fn permute(p: &mut [usize]) {
    let mut rng = thread_rng();
    for i in (1..p.len()).rev() {
        let target = rng.gen_range(0..i + 1);
        p.swap(i, target)
    }
}

fn perlin_generate_perm() -> Box<[usize]> {
    let mut p: Box<[usize]> = (0..256).collect();
    permute(&mut p);
    p
}

static NOISE: Lazy<Perlin> = Lazy::new(|| Perlin {
    // ranfloat: perlin_generate(),
    ranvec: perlin_generate(),
    perm_x: perlin_generate_perm(),
    perm_y: perlin_generate_perm(),
    perm_z: perlin_generate_perm(),
});

pub struct Noise {
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self { scale }
    }
}

impl Texture for Noise {
    fn value(&self, _uv: Vec2, p: &Vec3) -> Vec3 {
        // vec3(1., 1., 1.) * 0.5 * (1. + NOISE.noise(self.scale * p))
        // vec3(1., 1., 1.) * NOISE.turb(self.scale * p, 7)

        vec3(1., 1., 1.) * 0.5 * (1. + f64::sin(self.scale * p.z() + 10. * NOISE.turb(p, 7)))
    }
}
