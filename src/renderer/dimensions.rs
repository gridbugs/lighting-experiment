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

gfx_constant_struct!( WorldDimensions {
    world_size: [f32; 2] = "u_WorldSize",
    world_size_u32: [u32; 2] = "u_WorldSizeUint",
});

pub struct Dimensions<R: gfx::Resources> {
    pub fixed_dimensions: gfx::handle::Buffer<R, FixedDimensions>,
    pub output_dimensions: gfx::handle::Buffer<R, OutputDimensions>,
    pub world_dimensions: gfx::handle::Buffer<R, WorldDimensions>,
}

impl<R: gfx::Resources> Dimensions<R> {
    pub fn new<F>(factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        Self {
            fixed_dimensions: factory.create_constant_buffer(1),
            output_dimensions: factory.create_constant_buffer(1),
            world_dimensions: factory.create_constant_buffer(1),
        }
    }

    pub fn update_fixed_dimensions<C>(&self, sprite_sheet: &SpriteSheetTexture<R>, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.fixed_dimensions, &FixedDimensions {
            sprite_sheet_size: [sprite_sheet.width as f32, sprite_sheet.height as f32],
            cell_size: [input_sprite::WIDTH_PX as f32, input_sprite::HEIGHT_PX as f32],
        });
    }

    pub fn update_output_dimensions<C>(&self, target: &RenderTarget<R>, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.output_dimensions, &OutputDimensions {
            output_size: [target.width as f32, target.height as f32],
        });
    }

    pub fn update_world_dimensions<C>(&self, dimensions: (u32, u32), encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(&self.world_dimensions, &WorldDimensions {
            world_size: [dimensions.0 as f32, dimensions.1 as f32],
            world_size_u32: [dimensions.0, dimensions.1],
        });
    }

}
