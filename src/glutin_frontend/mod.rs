use gfx;
use gfx::Device;
use glutin;
use glutin::GlContext;
use gfx_window_glutin;
use gfx_device_gl;

use frontend::{Frontend, FrontendOutput, FrontendInput};

use renderer::{Renderer, ColourFormat, DepthFormat, RendererWorldState};

use input::InputEvent;

mod input;
use self::input::convert_event;

type Resources = gfx_device_gl::Resources;

pub struct GlutinFrontendOutput {
    window: glutin::GlWindow,
    device: gfx_device_gl::Device,
    renderer: Renderer<Resources>,
    encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>,
    factory: gfx_device_gl::Factory,
    rtv: gfx::handle::RenderTargetView<Resources, ColourFormat>,
}

pub struct GlutinFrontendInput {
    events_loop: glutin::EventsLoop,
}

pub type GlutinFrontend = Frontend<GlutinFrontendInput, GlutinFrontendOutput>;

pub fn create() -> GlutinFrontend {
    let builder = glutin::WindowBuilder::new()
        .with_fullscreen(glutin::get_primary_monitor());

    let events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();

    let (window, mut device, mut factory, rtv, _dsv) =
        gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

    let mut encoder = factory.create_command_buffer().into();

    let renderer = Renderer::new(&rtv, &mut factory, &mut encoder, &mut device);

    Frontend {
        input: GlutinFrontendInput {
            events_loop,
        },
        output: GlutinFrontendOutput {
            window,
            device,
            renderer,
            encoder,
            factory,
            rtv,
        },
    }
}

impl FrontendInput for GlutinFrontendInput {
    fn with_input<F: FnMut(InputEvent)>(&mut self, mut f: F) {
        self.events_loop.poll_events(|event| {
            if let Some(input_event) = convert_event(event) {
                f(input_event);
            }
        });
    }
}

impl<'a> FrontendOutput<'a> for GlutinFrontendOutput {
    type WorldState = RendererWorldState<'a, Resources>;
    fn with_world_state<F: FnMut(&mut Self::WorldState)>(&'a mut self, mut f: F) {
        let mut state = self.renderer.world_state(&mut self.factory);
        f(&mut state);
        state.finalise(&mut self.encoder);
    }
    fn draw(&mut self) {
        self.renderer.clear(&mut self.encoder);
        self.renderer.render(&mut self.encoder);

        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().expect("Failed to swap buffers");
        self.device.cleanup();
    }
}
