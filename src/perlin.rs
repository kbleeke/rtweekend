use core::f32;
use std::sync::Arc;

use nalgebra_glm::{dot, vec3, Vec3};
use once_cell::sync::Lazy;
use rand::{random, thread_rng, Rng};

use crate::texture::Texture;

pub struct Perlin {
    ranfloat: Box<[Vec3]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            ranfloat: generate(),
            perm_x: generate_perm(),
            perm_y: generate_perm(),
            perm_z: generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[vec3(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2i32 {
            for dj in 0..2i32 {
                for dk in 0..2i32 {
                    let idx = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.ranfloat[idx]
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    fn turb(&self, p: Vec3) -> f32 {
        self.turb_depth(p, 7)
    }

    fn turb_depth(&self, mut p: Vec3, depth: i32) -> f32 {
        let mut acc = 0.0;
        let mut weight = 1.0;
        for _i in 0..depth {
            acc += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }
        acc.abs()
    }
}

fn permute(p: &mut [usize]) {
    for i in (1..p.len()).rev() {
        let target = thread_rng().gen_range(0..i + 1);
        p.swap(target, i);
    }
}

fn generate() -> Box<[Vec3]> {
    (0..256)
        .map(|_| (vec3(-1.0, -1.0, -1.0) + 2.0 * random::<Vec3>()).normalize())
        .collect()
}

fn generate_perm() -> Box<[usize]> {
    let mut p: Vec<usize> = (0..256).collect();
    permute(&mut p);
    p.into_boxed_slice()
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v: Vec3 = vec3(u - i as f32, v - j as f32, w - k as f32);
                acc += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                    * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                    * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                    * dot(&c[i][j][k], &weight_v);
            }
        }
    }
    acc
}

static NOISE: Lazy<Perlin> = Lazy::new(|| Perlin::new());

pub struct NoiseTexture {
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Arc<Self> {
        Arc::new(Self { scale })
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        // vec3(1.0, 1.0, 1.0) * PERLIN.turb(scaled)
        vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + f32::sin(self.scale * p.z + 10.0 * NOISE.turb(*p)))
    }
}
