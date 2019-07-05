use cgmath::Vector3;
use cgmath::Vector4;

pub mod camera;
pub mod hitable_list;
pub mod material;
pub mod ray;
pub mod sphere;
#[allow(unused)]
pub mod vulkan;

pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
