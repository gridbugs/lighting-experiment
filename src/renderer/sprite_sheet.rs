use gfx;

use renderer::formats::ColourFormat;
use res::input_sprite;

const TILES_PER_WALL: u32 = 256;

pub struct SpriteSheet<R: gfx::Resources> {
    shader_resource_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    width: u32,
    height: u32,
}

struct SpriteSheetBuilder<R: gfx::Resources> {
    render_target_view: gfx::handle::RenderTargetView<R, ColourFormat>,
    shader_resource_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    width: u32,
    height: u32,
    input_sprites: Vec<input_sprite::InputSpritePixelCoord>,
}

impl<R: gfx::Resources> SpriteSheetBuilder<R> {
    fn new<F>(factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>
    {

        use self::input_sprite::InputSpritePixelCoord::*;

        let input_sprites = input_sprite::input_sprite_pixel_coords();

        let mut num_sprites = 0;

        for sprite in input_sprites.iter() {
            match sprite {
                &Simple { .. } => num_sprites += 1,
                &Wall { .. } => num_sprites += TILES_PER_WALL,
            }
        }

        let width = num_sprites * input_sprite::WIDTH_PX;
        let height = input_sprite::HEIGHT_PX;

        let (_, srv, rtv) = factory.create_render_target(width as u16, height as u16)
            .expect("Failed to create render target for sprite sheet");

        SpriteSheetBuilder {
            render_target_view: rtv,
            shader_resource_view: srv,
            width: width,
            height: height,
            input_sprites: input_sprites,
        }
    }

    fn populate(&mut self) {

    }

    fn build(self) -> SpriteSheet<R> {
        let Self { shader_resource_view, width, height, .. } = self;
        SpriteSheet {
            shader_resource_view,
            width,
            height,
        }
    }
}

impl<R: gfx::Resources> SpriteSheet<R> {
    pub fn new<F>(factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>
    {
        let mut builder = SpriteSheetBuilder::new(factory);
        builder.populate();
        builder.build()
    }
}
