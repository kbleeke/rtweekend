use std::{sync::Arc, time::Instant};

use nalgebra_glm::vec3;
use rand::random;
use rayon::prelude::*;

use raytrace::{camera::*, hitable_list::*, material::*, ray::*, sphere::*, Vec3};
use wgpu::{BackendBit, RequestAdapterOptions, RequestAdapterOptionsBase, Surface, util::{BufferInitDescriptor, DeviceExt}};
use winit::{event_loop::EventLoop, window::WindowBuilder};

const NX: i32 = 600;
const NY: i32 = 300;
const NS: i32 = 100;

fn random_scene() -> HitableList {
    let mut list: Vec<Box<dyn Hitable + Send>> = Vec::with_capacity(501);
    list.push(Sphere::boxed(
        vec3(0., -1000., 0.),
        1000.,
        Lambertian::new(vec3(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random();
            let center = vec3(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if vec3(4., 0.2, 0.).metric_distance(&center) > 0.9 {
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    Lambertian::new(random::<Vec3>().component_mul(&random::<Vec3>()))
                } else if choose_mat < 0.95 {
                    Metal::new(
                        0.5 * random::<Vec3>().add_scalar(1.0),
                        0.5 + random::<f32>(),
                    )
                } else {
                    Dielectric::new(1.5)
                };
                list.push(Sphere::boxed(center, 0.2, mat));
            }
        }
    }

    list.push(Sphere::boxed(vec3(0., 1., 0.), 1., Dielectric::new(1.5)));
    list.push(Sphere::boxed(
        vec3(-4., 1., 0.),
        1.,
        Lambertian::new(vec3(0.4, 0.2, 0.1)),
    ));
    list.push(Sphere::boxed(
        vec3(4., 1., 0.),
        1.,
        Metal::new(vec3(0.7, 0.6, 0.5), 0.0),
    ));

    HitableList::new(list)
}

fn color(r: &Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(mut rec) = world.hit(r, 0.001, std::f32::MAX) {
        match rec.material.scatter(r, &mut rec) {
            Some(ref res) if depth < 50 => {
                res.attenuation
                    .component_mul(&color(&res.scattered, world, depth + 1))
            }
            _ => vec3(0.0, 0.0, 0.0),
        }
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
    }
}

fn fill_buf(buffer: &mut Vec<[u8; 4]>) {
    let world = random_scene();
    println!("Scene generated");
    let start = Instant::now();

    let lookfrom = vec3(13., 2., 3.);
    let lookat = vec3(0., 0., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vec3(0., 1., 0.),
        20.,
        NX as f32 / NY as f32,
        aperture,
        dist_to_focus,
    );

    buffer.clear();

    (0..((NY) * NX))
        .into_par_iter()
        .map(|k| {
            let i = k % NX;
            let j = NY - k / NX;

            let mut col: Vec3 = (0..NS)
                .into_par_iter()
                .map(|_| {
                    let u = (i as f32 + random::<f32>()) / NX as f32;
                    let v = (j as f32 + random::<f32>()) / NY as f32;
                    let r = cam.get_ray(u, v);
                    color(&r, &world, 0)
                })
                .sum();

            col /= NS as f32;
            col = col.map(|c| c.sqrt());
            let col = col.map(|c| (255.99 * c) as u8);
            col.insert_row(3, 1).into()
        })
        .collect_into_vec(buffer);

    println!("Rendered in {:?}", start.elapsed());
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let size = window.inner_size();

    let instance = wgpu::Instance::new(BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let (device, mut queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: adapter.get_swap_chain_preferred_format(&surface),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });

    let swapchain = device.create_swap_chain(&surface, &sc_desc);

    let frame = swapchain.get_current_frame().unwrap().output;
}
