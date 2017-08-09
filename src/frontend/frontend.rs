use gfx;
use gfx::Device;
use glutin;
use glutin::GlContext;
use gfx_window_glutin;
use gfx_device_gl;

use entity_store::EntityStore;
use spatial_hash::SpatialHashTable;

use renderer::{Renderer, ColourFormat, DepthFormat};

pub struct Frontend {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    device: gfx_device_gl::Device,
    renderer: Renderer<gfx_device_gl::Resources>,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    factory: gfx_device_gl::Factory,
    rtv: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColourFormat>,
    dsv: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
}

impl Frontend {
    pub fn new() -> Self {
        let builder = glutin::WindowBuilder::new()
            .with_fullscreen(glutin::get_primary_monitor());

        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new();

        let (window, mut device, mut factory, rtv, dsv) =
            gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

        let mut encoder = factory.create_command_buffer().into();

        let renderer = Renderer::new(&rtv, &dsv, &mut factory, &mut encoder, &mut device);

        Frontend {
            events_loop,
            window,
            device,
            renderer,
            encoder,
            factory,
            rtv,
            dsv,
        }
    }

    pub fn spin(&mut self, entity_store: &EntityStore, spatial_hash: &SpatialHashTable) {

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
            self.encoder.clear_depth(&self.dsv, 1.0);

            self.renderer.render(entity_store, spatial_hash, &mut self.factory, &mut self.encoder);

            self.encoder.flush(&mut self.device);
            self.window.swap_buffers().expect("Failed to swap buffers");
            self.device.cleanup();
        }
    }
}
