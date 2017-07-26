use std::thread;
use std::time::Duration;
use std::result;

use gfx;
use gfx::Device;
use glutin;
use glutin::GlContext;
use gfx_window_glutin;
use gfx_device_gl;

use renderer::formats::{ColourFormat, DepthFormat};
use renderer::tile_map;

#[derive(Debug)]
pub enum Error {
    RendererError,
}

pub type Result<T> = result::Result<T, Error>;

pub struct Frontend {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    device: gfx_device_gl::Device,
    tile_map: gfx::Bundle<gfx_device_gl::Resources, tile_map::tile_map::Data<gfx_device_gl::Resources>>,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    rtv: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColourFormat>,
}

impl Frontend {
    pub fn new() -> Result<Self> {
        let builder = glutin::WindowBuilder::new()
            .with_fullscreen(glutin::get_primary_monitor());

        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new();

        let (window, device, mut factory, rtv, _dsv) =
            gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

        let encoder = factory.create_command_buffer().into();
        let tile_map_bundle = tile_map::init(rtv.clone(), &mut factory).map_err(|_| Error::RendererError)?;

        Ok(Frontend {
            events_loop: events_loop,
            window: window,
            device: device,
            tile_map: tile_map_bundle,
            encoder: encoder,
            rtv: rtv,
        })
    }

    pub fn spin(&mut self) {
        let mut running = true;
        while running {

            self.encoder.clear(&self.rtv, [0.0, 0.0, 0.0, 1.0]);
            self.tile_map.encode(&mut self.encoder);
            self.encoder.flush(&mut self.device);
            self.window.swap_buffers().expect("Failed to swap buffers");
            self.device.cleanup();

            thread::sleep(Duration::from_millis(16));

            self.events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => running = false,
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }
}
