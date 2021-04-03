pub mod vec2;
mod vec3;

use std::f64::consts::PI;

use rand::random;
pub use vec2::{vec2, Vec2};
pub use vec3::*;

mod onb;
pub use onb::Onb;

pub fn random_cosine_direction() -> Vec3 {
    let r1: f64 = random();
    let r2: f64 = random();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let x = f64::cos(phi) * r2.sqrt();
    let y = f64::sin(phi) * r2.sqrt();

    vec3(x, y, z)
}
