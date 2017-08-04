use std::result;

use gfx;
use gfx::Device;
use glutin;
use glutin::GlContext;
use gfx_window_glutin;
use gfx_device_gl;

use renderer::formats::{ColourFormat, DepthFormat};
use renderer::example;
use renderer::sprite_sheet::SpriteSheet;

#[derive(Debug)]
pub enum Error {
    RendererError,
}

pub type Result<T> = result::Result<T, Error>;

pub struct Frontend {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    device: gfx_device_gl::Device,
    example: example::Example<gfx_device_gl::Resources>,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    factory: gfx_device_gl::Factory,
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
        let example = example::Example::new(rtv.clone(), &mut factory).map_err(|_| Error::RendererError)?;

        let sprite_sheet: SpriteSheet<gfx_device_gl::Resources> = SpriteSheet::new(&mut factory);

        Ok(Frontend {
            events_loop: events_loop,
            window: window,
            device: device,
            example: example,
            encoder: encoder,
            factory: factory,
            rtv: rtv,
        })
    }

    pub fn spin(&mut self) {
        let mut running = true;
        while running {

            self.events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => running = false,
                        _ => (),
                    },
                    _ => (),
                }
            });

            self.encoder.clear(&self.rtv, [0.0, 0.0, 0.0, 1.0]);

            self.example.update(&mut self.encoder, &mut self.factory);
            self.example.draw(&mut self.encoder);

            self.encoder.flush(&mut self.device);
            self.window.swap_buffers().expect("Failed to swap buffers");
            self.device.cleanup();
        }
    }
}
