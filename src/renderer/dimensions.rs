use gfx;

use renderer::render_target::RenderTarget;
use renderer::sprite_sheet::SpriteSheetTexture;

use res::input_sprite;

gfx_constant_struct!( FixedDimensions {
    sprite_sheet_size: [f32; 2] = "u_SpriteSheetSize",
    cell_size: [f32; 2] = "u_CellSize",
});

gfx_constant_struct!( OutputDimensions {
    output_size: [f32; 2] = "u_OutputSize",
});

pub struct Dimensions<R: gfx::Resources> {
    pub fixed_dimensions: gfx::handle::Buffer<R, FixedDimensions>,
    pub output_dimensions: gfx::handle::Buffer<R, OutputDimensions>,
}

impl<R: gfx::Resources> Dimensions<R> {
    pub fn new<F>(factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        Self {
            fixed_dimensions: factory.create_constant_buffer(1),
            output_dimensions: factory.create_constant_buffer(1),
        }
    }

    pub fn update_all<C>(&self, target: &RenderTarget<R>, sprite_sheet: &SpriteSheetTexture<R>, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.fixed_dimensions, &FixedDimensions {
            sprite_sheet_size: [sprite_sheet.width as f32, sprite_sheet.height as f32],
            cell_size: [input_sprite::WIDTH_PX as f32, input_sprite::HEIGHT_PX as f32],
        });

        self.update_output_dimensions(target, encoder);
    }

    pub fn update_output_dimensions<C>(&self, target: &RenderTarget<R>, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.output_dimensions, &OutputDimensions {
            output_size: [target.width as f32, target.height as f32],
        });
    }
}
