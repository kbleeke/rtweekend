pub mod camera;
pub mod hit;
pub mod material;
pub mod perlin;
pub mod shapes;
pub mod texture;

use hit::{
    hittable::flip_normals,
    instance::{rotate_y, translate},
};
use nalgebra::zero;
use nalgebra_glm::{vec3, vec3_to_vec4, vec4};
use perlin::NoiseTexture;
use rand::random;
use rayon::prelude::*;
use shapes::{
    cuboid::Cuboid,
    rect::{XyRect, XzRect, YzRect},
};
use std::{f32::INFINITY, time::Instant};
use texture::CheckerTexture;

use crate::hit::hittable::{Hittable, HittableList};
use crate::texture::ImageTexture;
use crate::{camera::*, hit::ray::*, material::*, shapes::sphere::*, texture::ConstantTexture};

pub use nalgebra_glm::{Vec3, Vec4};

pub const NX: i32 = 300;
pub const NY: i32 = 300;
pub const NS: i32 = 800;

pub const SPHERES: i32 = 5;

pub fn random_scene() -> HittableList {
    let mut list: Vec<Box<dyn Hittable>> = Vec::with_capacity(501);
    let checker = CheckerTexture::new(
        ConstantTexture::new(vec3(0.2, 0.3, 0.1)),
        ConstantTexture::new(vec3(0.9, 0.9, 0.9)),
    );
    list.push(Sphere::boxed(
        vec3(0., -1000., 0.),
        1000.,
        Lambertian::new(checker),
    ));

    for a in -SPHERES..SPHERES {
        for b in -SPHERES..SPHERES {
            let choose_mat: f32 = random();
            let center = vec3(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if vec3(4., 0.2, 0.).metric_distance(&center) > 0.9 {
                if choose_mat < 0.8 {
                    let mat = Lambertian::new(ConstantTexture::new(
                        random::<Vec3>().component_mul(&random::<Vec3>()),
                    ));
                    let center1 = center + vec3(0.0, 0.5 * random::<f32>(), 0.0);
                    list.push(MovingSphere::boxed(center, center1, 0.0, 1.0, 0.2, mat));
                } else if choose_mat < 0.95 {
                    let mat = Metal::new(
                        ConstantTexture::new(0.5 * random::<Vec3>().add_scalar(1.0)),
                        0.5 + random::<f32>(),
                    );
                    list.push(Sphere::boxed(center, 0.2, mat));
                } else {
                    let mat = Dielectric::new(1.5);
                    list.push(Sphere::boxed(center, 0.2, mat));
                };
            }
        }
    }

    list.push(Sphere::boxed(vec3(0., 1., 0.), 1., Dielectric::new(1.5)));
    list.push(Sphere::boxed(
        vec3(-4., 1., 0.),
        1.,
        Lambertian::new(ConstantTexture::new(vec3(0.4, 0.2, 0.1))),
    ));
    list.push(Sphere::boxed(
        vec3(4., 1., 0.),
        1.,
        Metal::new(ConstantTexture::new(vec3(0.7, 0.6, 0.5)), 0.0),
    ));

    HittableList::new(list)
}

pub fn two_perlin_spheres() -> HittableList {
    let tex = NoiseTexture::new(4.0);
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Sphere::boxed(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(tex.clone()),
    ));
    list.push(Sphere::boxed(
        vec3(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(tex),
    ));

    HittableList::new(list)
}

pub fn earth() -> Sphere {
    let image = ::image::open("earthmap.jpg").expect("earthmap").into_rgb8();

    let tex = ImageTexture::new(image);
    Sphere::new(vec3(0.0, 0.0, 0.0), 2.0, Lambertian::new(tex))
}

pub fn simple_light() -> HittableList {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    let noise = NoiseTexture::new(4.0);
    list.push(Sphere::boxed(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(noise.clone()),
    ));
    list.push(Sphere::boxed(
        vec3(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(noise.clone()),
    ));
    list.push(Sphere::boxed(
        vec3(0.0, 7.0, 0.0),
        2.0,
        DiffuseLight::new(ConstantTexture::new(vec3(4.0, 4.0, 4.0))),
    ));
    list.push(XyRect::boxed(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        DiffuseLight::new(ConstantTexture::new(vec3(4.0, 4.0, 4.0))),
    ));

    HittableList::new(list)
}

pub fn cornell_box() -> Box<dyn Hittable> {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    let red = Lambertian::new(ConstantTexture::new(vec3(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(vec3(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture::new(vec3(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(ConstantTexture::new(vec3(15., 15., 15.)));

    list.push(flip_normals(YzRect::new(0., 555., 0., 555., 555., green)));
    list.push(YzRect::boxed(0., 555., 0., 555., 0., red));
    list.push(XzRect::boxed(213., 343., 227., 332., 554., light));

    list.push(flip_normals(XzRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    list.push(XzRect::boxed(0., 555., 0., 555., 0., white.clone()));
    list.push(flip_normals(XyRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    list.push(Box::new(translate(
        rotate_y(
            Cuboid::new(vec3(0., 0., 0.), vec3(165., 165., 165.), white.clone()),
            -18.,
        ),
        vec3(130., 0., 65.),
    )));

    list.push(Box::new(translate(
        rotate_y(
            Cuboid::new(vec3(0., 0., 0.), vec3(165., 330., 165.), white.clone()),
            15.,
        ),
        vec3(265., 0., 295.),
    )));

    Box::new(HittableList::new(list))
}

pub fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return zero();
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted: Vec3 = rec.material.emitted(rec.uv.u, rec.uv.v, &rec.p);
        let scattered = rec
            .material
            .scatter(r, &rec)
            .map(|scattered| {
                let new_color: Vec3 = color(&scattered.ray, world, depth - 1);
                scattered.attenuation.component_mul(&new_color)
            })
            .unwrap_or(zero());
        emitted + scattered
    } else {
        zero()
    }
}

pub fn fill_buf(buffer: &mut Vec<[u8; 4]>) {
    // let world = random_scene();
    // let world = BvhNode::new(world.into_inner(), 0.0, 0.0);
    // let world = two_perlin_spheres();
    // let world = earth();
    // let world = simple_light();
    let world = cornell_box();

    println!("Scene generated");
    let start = Instant::now();

    let lookfrom = vec3(278., 278., -800.);
    let lookat = vec3(278., 278., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vec3(0., 1., 0.),
        40.,
        NX as f32 / NY as f32,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
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
                    let u = (i as f32 + random::<f32>()) / (NX - 1) as f32;
                    let v = (j as f32 + random::<f32>()) / (NY - 1) as f32;
                    let r = cam.get_ray(u, v);
                    color(&r, world.as_ref(), 50)
                })
                .sum();

            col = col.map(|v| if v.is_nan() { 0.0 } else { v });
            let scale = 1.0 / NS as f32;
            col = col.map(|v| f32::sqrt(scale * v));
            let col = col.map(|v| (256. * v.clamp(0.0, 0.999)) as u8);
            vec4(col.x, col.y, col.z, 0).into()
        })
        .collect_into_vec(buffer);

    println!("Rendered in {:?}", start.elapsed());
}
