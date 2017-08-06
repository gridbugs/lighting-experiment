use std::result;

use gfx;
use gfx::Device;
use glutin;
use glutin::GlContext;
use gfx_window_glutin;
use gfx_device_gl;
use image;

use renderer::formats::{ColourFormat, DepthFormat};
use renderer::sprite_sheet::SpriteSheet;
use renderer::tile_renderer::TileRenderer;

use res::{input_sprite, paths, files};

#[derive(Debug)]
pub enum Error {
    RendererError,
}

pub type Result<T> = result::Result<T, Error>;

pub struct Frontend {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    device: gfx_device_gl::Device,
    renderer: TileRenderer<gfx_device_gl::Resources>,
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

        let (window, mut device, mut factory, rtv, _dsv) =
            gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

        let mut encoder = factory.create_command_buffer().into();

        let sprite_sheet_path = paths::res_path(files::SPRITE_SHEET);
        let image = image::open(&sprite_sheet_path)
            .expect(format!("Failed to open sprite sheet (looked for {})", sprite_sheet_path.display()).as_ref())
            .to_rgba();
        let sprite_sheet: SpriteSheet<gfx_device_gl::Resources> =
            SpriteSheet::new(image, input_sprite::input_sprite_pixel_coords(), &mut factory, &mut encoder, &mut device);

        let renderer = TileRenderer::new(sprite_sheet, rtv.clone(), &mut factory);

        Ok(Frontend {
            events_loop,
            window,
            device,
            renderer,
            encoder,
            factory,
            rtv,
        })
    }

    pub fn spin(&mut self) {
        self.renderer.init(&mut self.encoder);
        self.renderer.update(&mut self.encoder, &mut self.factory);
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

            self.renderer.draw(&mut self.encoder);

            self.encoder.flush(&mut self.device);
            self.window.swap_buffers().expect("Failed to swap buffers");
            self.device.cleanup();
        }
    }
}
