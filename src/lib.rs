use std::{f64::consts::FRAC_PI_4, sync::Arc};

use camera::Camera;
use hit::{Hitable, Ray};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use math::{vec2, vec3, Vec3};
use objects::sphere::Sphere;
use objects::{cuboid::Cuboid, rect::XyRect};
use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use texture::{Constant, Noise};
use transform::HitableExt;
use volume::ConstantMedium;

pub mod camera;
pub mod hit;
pub mod materials;
pub mod math;
pub mod objects;
pub mod texture;
pub mod transform;
pub mod volume;

fn color(r: &Ray, world: &dyn Hitable, depth: usize) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.material.emitted(rec.uv, &rec.p);

        return if depth <= 0 {
            Vec3::zero()
        } else if let Some(scatter) = rec.material.scatter(r, &rec) {
            emitted
                + scatter.attenuation
                    * rec.material.scattering_pdf(r, &rec, &scatter.scattered)
                    * color(&scatter.scattered, world, depth - 1)
                    / scatter.pdf
        } else {
            emitted
        };
    }
    Vec3::zero()
}

pub fn two_spheres() -> Box<dyn Hitable> {
    Box::new([
        Sphere::new(
            vec3(0., 0., -1.),
            0.5,
            Arc::new(Lambertian::constant(vec3(0.5, 0.7, 1.0))),
        ),
        Sphere::new(
            vec3(0., -100.5, -1.),
            100.,
            Arc::new(Lambertian::constant(vec3(0.5, 0.7, 1.0))),
        ),
    ])
}

pub fn four_spheres() -> Box<dyn Hitable> {
    Box::new([
        Sphere::new(
            vec3(0., 0., -1.),
            0.5,
            Lambertian::constant(vec3(0.1, 0.2, 0.5)),
        ),
        Sphere::new(
            vec3(0., -100.5, -1.),
            100.,
            Lambertian::constant(vec3(0.8, 0.8, 0.)),
        ),
        Sphere::new(vec3(1., 0., -1.), 0.5, Metal::new(vec3(0.8, 0.6, 0.2), 0.3)),
        Sphere::new(vec3(-1., 0., -1.), 0.5, Dielectric::new(1.5)),
        Sphere::new(vec3(-1., 0., -1.), -0.45, Dielectric::new(1.5)),
    ])
}

pub fn cam_test() -> Box<dyn Hitable> {
    let r = FRAC_PI_4.cos();
    Box::new([
        Sphere::new(vec3(-r, 0., -1.), r, Lambertian::constant(vec3(0., 0., 1.))),
        Sphere::new(vec3(r, 0., -1.), r, Lambertian::constant(vec3(1., 0., 0.))),
    ])
}

pub fn two_perlin_spheres() -> Box<dyn Hitable> {
    let noise = Arc::new(Noise::new(4.));
    Box::new([
        Sphere::new(vec3(0., -1000., 0.), 1000., Lambertian::new(noise.clone())),
        Sphere::new(vec3(0., 2., 0.), 2., Lambertian::new(noise.clone())),
    ])
}

pub fn simple_light() -> Box<dyn Hitable> {
    let noise = Arc::new(Noise::new(4.));
    Box::new([
        Box::new(Sphere::new(
            vec3(0., -1000., 0.),
            1000.,
            Lambertian::new(noise.clone()),
        )) as Box<dyn Hitable>,
        Box::new(Sphere::new(
            vec3(0., 2., 0.),
            2.,
            Lambertian::new(noise.clone()),
        )),
        Box::new(XyRect::new(
            vec2(3., 5.),
            vec2(1., 3.),
            -2.,
            DiffuseLight::new(Constant::new(vec3(4., 4., 4.))),
        )),
    ])
}

use crate::objects::rect::{XzRect, YzRect};
pub fn cornell_box() -> Box<dyn Hitable> {
    let red = Lambertian::constant(vec3(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::constant(vec3(0.73, 0.73, 0.73)));
    let green = Lambertian::constant(vec3(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Constant::new(vec3(15., 15., 15.)));

    Box::new([
        YzRect::new(vec2(0., 555.), vec2(0., 555.), 555., green).boxed() as Box<dyn Hitable>,
        YzRect::new(vec2(0., 555.), vec2(0., 555.), 0., red).boxed(),
        XzRect::new(vec2(213., 343.), vec2(227., 332.), 554., light).boxed(),
        XzRect::new(vec2(0., 555.), vec2(0., 555.), 555., white.clone()).boxed(),
        XzRect::new(vec2(0., 555.), vec2(0., 555.), 0., white.clone()).boxed(),
        XyRect::new(vec2(0., 555.), vec2(0., 555.), 555., white.clone()).boxed(),
        Cuboid::new(vec3(0., 0., 0.), vec3(165., 330., 165.), white.clone())
            .rotate_y(15.)
            .translate(vec3(265., 0., 295.))
            .boxed(),
        Cuboid::new(vec3(0., 0., 0.), vec3(165., 165., 165.), white.clone())
            .rotate_y(-18.)
            .translate(vec3(130., 0., 65.))
            .boxed(),
    ])
}

pub fn cornell_smoke() -> Box<dyn Hitable> {
    let red = Lambertian::constant(vec3(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::constant(vec3(0.73, 0.73, 0.73)));
    let green = Lambertian::constant(vec3(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Constant::new(vec3(7., 7., 7.)));

    Box::new([
        YzRect::new(vec2(0., 555.), vec2(0., 555.), 555., green).boxed() as Box<dyn Hitable>,
        YzRect::new(vec2(0., 555.), vec2(0., 555.), 0., red).boxed(),
        XzRect::new(vec2(113., 443.), vec2(127., 432.), 554., light).boxed(),
        XzRect::new(vec2(0., 555.), vec2(0., 555.), 555., white.clone()).boxed(),
        XzRect::new(vec2(0., 555.), vec2(0., 555.), 0., white.clone()).boxed(),
        XyRect::new(vec2(0., 555.), vec2(0., 555.), 555., white.clone()).boxed(),
        ConstantMedium::new(
            0.01,
            Cuboid::new(vec3(0., 0., 0.), vec3(165., 330., 165.), white.clone())
                .rotate_y(15.)
                .translate(vec3(265., 0., 295.))
                .boxed(),
            Constant::new(vec3(0., 0., 0.)),
        )
        .boxed(),
        ConstantMedium::new(
            0.01,
            Cuboid::new(vec3(0., 0., 0.), vec3(165., 165., 165.), white.clone())
                .rotate_y(-18.)
                .translate(vec3(130., 0., 65.))
                .boxed(),
            Constant::new(vec3(1., 1., 1.)),
        )
        .boxed(),
    ])
}

pub struct Scene {
    pub world: Box<dyn Hitable>,
    pub cam: Camera,
}

impl Scene {
    pub fn fill_buf(&self, nx: usize, ny: usize, ns: usize) -> Vec<[u8; 4]> {
        let cam = &self.cam;
        let world = &self.world;

        let n = ny * nx;

        let progress = ProgressBar::new(n as u64);
        progress.set_draw_delta(nx as u64);
        progress.set_style(ProgressStyle::default_bar().template(
            "[{elapsed_precise}] [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        ));

        let vec = (0..n)
            .into_par_iter()
            .progress_with(progress.clone())
            .map(|n| {
                let i = n % nx;
                let j = ny - n / nx;

                let mut rng = thread_rng();

                let col: Vec3 = (0..ns)
                    .map(|_s| {
                        let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / ny as f64;

                        let ray = cam.get_ray(u, v);
                        color(&ray, world.as_ref(), 50)
                    })
                    .sum::<Vec3>();

                let col = col.map(|c| if c.is_nan() { 0.0 } else { c });
                let col = col.map(|c| f64::sqrt(c / ns as f64));

                let ir = (255.99 * col[0]) as u8;
                let ig = (255.99 * col[1]) as u8;
                let ib = (255.99 * col[2]) as u8;

                [ir, ig, ib, 0]
            })
            .collect();

        progress.finish();
        vec
    }
}
