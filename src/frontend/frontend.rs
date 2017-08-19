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
}

impl Frontend {
    pub fn new() -> Self {
        let builder = glutin::WindowBuilder::new()
            .with_fullscreen(glutin::get_primary_monitor());

        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new();

        let (window, mut device, mut factory, rtv, _dsv) =
            gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

        let mut encoder = factory.create_command_buffer().into();

        let renderer = Renderer::new(&rtv, &mut factory, &mut encoder, &mut device);

        Frontend {
            events_loop,
            window,
            device,
            renderer,
            encoder,
            factory,
            rtv,
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

            self.renderer.clear(&mut self.encoder);

            let player_id = entity_store.player.iter().next().expect("Failed to find player");
            let player_position = entity_store.position.get(&player_id).expect("Failed to find player position");
            self.renderer.update_offset(*player_position, &mut self.encoder);

            self.renderer.render(entity_store, spatial_hash, &mut self.factory, &mut self.encoder);

            self.encoder.flush(&mut self.device);
            self.window.swap_buffers().expect("Failed to swap buffers");
            self.device.cleanup();

        }
    }
}
