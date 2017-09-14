use gfx;
use gfx::Device;
use glutin;
use glutin::GlContext;
use gfx_window_glutin;
use gfx_device_gl;

use frontend::{FrontendOutput, FrontendInput, VisibleRange};

use renderer::{Renderer, ColourFormat, DepthFormat, RendererWorldState};

use input::Input;

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
    dsv: gfx::handle::DepthStencilView<Resources, DepthFormat>,
}

pub struct GlutinFrontendInput {
    events_loop: glutin::EventsLoop,
}

pub fn create() -> (GlutinFrontendInput, GlutinFrontendOutput) {
    let builder = glutin::WindowBuilder::new();

    let events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);

    let (window, mut device, mut factory, rtv, dsv) =
        gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

    let mut encoder = factory.create_command_buffer().into();

    let renderer = Renderer::new(&rtv, &mut factory, &mut encoder, &mut device);

    let input = GlutinFrontendInput {
        events_loop,
    };

    let output = GlutinFrontendOutput {
        window,
        device,
        renderer,
        encoder,
        factory,
        rtv,
        dsv,
    };

    (input, output)
}

impl FrontendInput for GlutinFrontendInput {
    fn with_input<F: FnMut(Input)>(&mut self, mut f: F) {
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
    fn handle_resize(&mut self, _width: u16, _height: u16) {
        let (rtv, dsv) = gfx_window_glutin::new_views(&self.window);
        self.renderer.handle_resize(&rtv, &mut self.encoder, &mut self.factory);
        self.rtv = rtv;
        self.dsv = dsv;
    }
    fn update_world_size(&mut self, width: u32, height: u32) {
        self.renderer.update_world_size(width, height, &mut self.encoder);
    }
    fn visible_range(&self) -> VisibleRange {
        self.renderer.visible_range()
    }
}
