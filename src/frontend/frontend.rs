use gfx;
use gfx::Device;
use glutin;
use glutin::GlContext;
use gfx_window_glutin;
use gfx_device_gl;

use entity_store::EntityStore;
use spatial_hash::SpatialHashTable;

use renderer::{Renderer, ColourFormat, DepthFormat, RendererFrame};

use input::InputEvent;
use frontend::input::convert_event;

type Resources = gfx_device_gl::Resources;
pub type FrontendOutputFrame<'a> = RendererFrame<'a, Resources>;
pub type FrontendInputEvent = InputEvent;

pub struct FrontendInput {
    events_loop: glutin::EventsLoop,
}

pub struct FrontendOutput {
    window: glutin::GlWindow,
    device: gfx_device_gl::Device,
    renderer: Renderer<Resources>,
    encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>,
    factory: gfx_device_gl::Factory,
    rtv: gfx::handle::RenderTargetView<Resources, ColourFormat>,
}

pub struct Frontend {
    pub input: FrontendInput,
    pub output: FrontendOutput,
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
            input: FrontendInput {
                events_loop,
            },
            output: FrontendOutput {
                window,
                device,
                renderer,
                encoder,
                factory,
                rtv,
            },
        }
    }
}

impl FrontendInput {
    pub fn with_input<F: FnMut(FrontendInputEvent)>(&mut self, mut f: F) {
        self.events_loop.poll_events(|event| {
            if let Some(input_event) = convert_event(event) {
                f(input_event);
            }
        });
    }
}

impl FrontendOutput {
    pub fn init(&mut self, entity_store: &EntityStore, spatial_hash: &SpatialHashTable) {
        self.renderer.update_all(entity_store, spatial_hash, &mut self.factory);
    }

    pub fn with_frame<F: FnMut(&mut FrontendOutputFrame)>(&mut self, mut f: F) {
        let maybe_offset = {
            let mut frame = self.renderer.frame(&mut self.factory);
            f(&mut frame);
            frame.finalise()
        };

        if let Some(offset) = maybe_offset {
            self.renderer.update_offset(offset, &mut self.encoder);
        }

        self.draw();
    }

    pub fn draw(&mut self) {
        self.renderer.clear(&mut self.encoder);
        self.renderer.render(&mut self.encoder);

        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().expect("Failed to swap buffers");
        self.device.cleanup();
    }
}
