use std::time::Instant;
use std::sync::Arc;

use cgmath::{prelude::*, vec3, Vector3, conv::array3};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};
use rand::random;
use rayon::prelude::*;

use camera::*;
use hitable_list::*;
use material::*;
use ray::*;
use sphere::*;

mod camera;
mod hitable_list;
mod material;
mod ray;
mod sphere;

const NX: i32 = 200;
const NY: i32 = 100;
const NS: i32 = 100;

type Vec3 = Vector3<f32>;

fn random_scene() -> HitableList {
    let mut list: Vec<Box<dyn Hitable + Send>> = Vec::with_capacity(501);
    list.push(Sphere::boxed(vec3(0., -1000., 0.), 1000., Lambertian::new(vec3(0.5, 0.5, 0.5))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random();
            let center = vec3(a as f32 + 0.9 * random::<f32>(), 0.2, b as f32 + 0.9 * random::<f32>());
            
            if vec3(4., 0.2, 0.).distance(center) > 0.9 {
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    Lambertian::new(random::<Vec3>().mul_element_wise(random::<Vec3>()))
                } else if choose_mat < 0.95 {
                    Metal::new(0.5 * random::<Vec3>().add_element_wise(1.0), 0.5 + random::<f32>())
                } else {
                    Dielectric::new(1.5)
                };
                list.push(Sphere::boxed(center, 0.2, mat));
            }
        }
    }

    list.push(Sphere::boxed(vec3(0., 1., 0.), 1., Dielectric::new(1.5)));
    list.push(Sphere::boxed(vec3(-4., 1., 0.), 1., Lambertian::new(vec3(0.4, 0.2, 0.1))));
    list.push(Sphere::boxed(vec3(4., 1., 0.), 1., Metal::new(vec3(0.7, 0.6, 0.5), 0.0)));

    HitableList::new(list)
}

fn color(r: &Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(mut rec) = world.hit(r, 0.001, std::f32::MAX) {
        match rec.material.scatter(r, &mut rec) {
            Some(ref res) if depth < 50 => {
                res.attenuation
                    .mul_element_wise(color(&res.scattered, world, depth + 1))
            }
            _ => vec3(0.0, 0.0, 0.0),
        }
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
    }
}

fn fill_buf(buffer: &mut Vec<[u8; 3]>) {
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

    (0..((NY-1) * NX)).into_par_iter().map(|k| {
        let i = k % NX;
        let j = k / NX;

        let mut col: Vec3 = (0..NS).into_par_iter().map(|_| {
            let u = (i as f32 + random::<f32>()) / NX as f32;
            let v = (j as f32 + random::<f32>()) / NY as f32;
            let r = cam.get_ray(u, v);
            color(&r, &world, 0)
        }).sum();

        col /= NS as f32;
        col = col.map(|c| c.sqrt());
        let col = col.map(|c| (255.99 * c) as u8);
        array3(col)
    }).collect_into_vec(buffer);

    println!("Rendered in {:?}", start.elapsed());
}

fn draw_buf(buffer: &mut Vec<[u8; 3]>, fbo: u32, tex: u32, window: &Window) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            NX,
            NY,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            buffer.as_ptr() as *const _,
        );
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, fbo);
        let window_size = window.inner_size();
        gl::BlitFramebuffer(
            0,
            0,
            NX,
            NY,
            0,
            0,
            window_size.width as i32,
            window_size.height as i32,
            gl::COLOR_BUFFER_BIT,
            gl::LINEAR,
        );
    }
}

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glutin::dpi::LogicalSize::new(800., 400.));
    let windowed_context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    let mut fbo = 0_u32;
    let mut tex = 0_u32;

    unsafe {
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGB8, NX, NY);
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, fbo);
        gl::FramebufferTexture2D(
            gl::READ_FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            tex,
            0,
        )
    }

    let mut buffer = Vec::with_capacity((NX * NY * 3) as usize);
    fill_buf(&mut buffer);
    draw_buf(&mut buffer, fbo, tex, windowed_context.window());
    windowed_context.swap_buffers().unwrap();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    windowed_context.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::RedrawRequested => {
                    // fill_buf(&mut buffer);
                    // draw_buf(&mut buffer, fbo, tex, windowed_context.window());
                    // windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
