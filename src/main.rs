use std::thread;

use clap::Clap;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Debug, clap::Clap)]
struct Opts {
    #[clap(default_value = "500")]
    ns: usize,
    #[clap(default_value = "500")]
    nx: usize,
    #[clap(default_value = "500")]
    ny: usize,
}

fn main() {
    let Opts { ns, nx, ny } = dbg!(Opts::parse());

    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    let event_loop = EventLoop::with_user_event();
    let event_proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(nx as u32, ny as u32))
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size();

    let surface = pixels::SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = pixels::PixelsBuilder::new(nx as u32, ny as u32, surface)
        .texture_format(wgpu::TextureFormat::Rgba8UnormSrgb)
        .build()
        .unwrap();

    let scene = raytrace2::cornell_specular(nx, ny);

    thread::spawn(move || {
        let buffer = scene.fill_buf(nx, ny, ns);
        event_proxy.send_event(buffer).unwrap();
    });

    event_loop.run(move |event, _target, control| match event {
        Event::UserEvent(buffer) => {
            let frame = pixels.get_frame();
            let buffer = bytemuck::cast_slice(&buffer);
            frame.copy_from_slice(buffer);
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control = ControlFlow::Exit,
        Event::WindowEvent {
            event: WindowEvent::Resized(new_size),
            ..
        } => {
            println!("resize {:?}", new_size);
            pixels.resize_surface(new_size.width, new_size.height);
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            println!("rendering");
            pixels.render().unwrap();
        }
        _ => (),
    });
}
