use glutin;
use gfx_window_glutin;

use glutin_frontend::formats::{ColourFormat, DepthFormat};

pub struct GlutinFrontend {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
}

impl GlutinFrontend {
    pub fn new() -> Self {
        let builder = glutin::WindowBuilder::new()
            .with_fullscreen(glutin::get_primary_monitor());

        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new();

        let (window, _device, _factory, _rtv, _dsv) =
            gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

        GlutinFrontend {
            events_loop: events_loop,
            window: window,
        }
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
        }
    }
}
