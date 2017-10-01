use gfx;
use renderer::formats::{ColourFormat, DepthFormat};
use res::input_sprite;

const NUM_ROWS: u16 = 15;
const HEIGHT_PX: u16 = NUM_ROWS * input_sprite::HEIGHT_PX as u16;

pub struct RenderTarget<R: gfx::Resources> {
    pub rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
    pub dsv: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub srv: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub width: u16,
    pub height: u16,
    pub num_rows: u16,
}

impl<R: gfx::Resources> RenderTarget<R> {
    pub fn new<F>(scaled_window_dimensions: (u16, u16), factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let width = ((scaled_window_dimensions.0 as u32 * HEIGHT_PX as u32) /
            scaled_window_dimensions.1 as u32) as u16;

        let (_, srv, rtv) = factory.create_render_target(width, HEIGHT_PX)
            .expect("Failed to create render target");
        let (_, _, dsv) = factory.create_depth_stencil(width, HEIGHT_PX)
            .expect("Failed to create depth stencil");

        Self {
            rtv,
            dsv,
            srv,
            width,
            height: HEIGHT_PX,
            num_rows: NUM_ROWS,
        }
    }
}
