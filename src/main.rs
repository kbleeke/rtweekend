use std::thread;

use pixels::Pixels;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use raytrace::{fill_buf, NX, NY};

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .unwrap();

    let event_loop = EventLoop::with_user_event();
    let event_proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(NX as u32, NY as u32))
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size();

    let surface = pixels::SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(NX as u32, NY as u32, surface).unwrap();

    thread::spawn(move || {
        let mut buffer = Vec::new();
        fill_buf(&mut buffer);
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
        Event::RedrawRequested(_) => {
            pixels.render().unwrap();
        }
        _ => (),
    });
}
