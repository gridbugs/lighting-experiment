use gfx;
use image::RgbaImage;

use renderer::formats::ColourFormat;
use res::input_sprite::{self, InputSpritePixelCoord};
use content::sprite::{self, Sprite};

const TILES_PER_WALL: u32 = 256;

pub struct SpriteResolution;
impl Default for SpriteResolution {
    fn default() -> Self {
        SpriteResolution
    }
}

pub struct SpriteSheet<R: gfx::Resources> {
    shader_resource_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    width: u32,
    height: u32,
    sprite_table: Vec<SpriteResolution>,
}

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
    tex: [f32; 2] = "a_Tex",
});

gfx_pipeline!( pipe {
    vertex: gfx::VertexBuffer<Vertex> = (),
    out: gfx::RenderTarget<ColourFormat> = "Target0",
});

struct SpriteSheetBuilder<R: gfx::Resources> {
    render_target_view: gfx::handle::RenderTargetView<R, ColourFormat>,
    shader_resource_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    width: u32,
    height: u32,
    input_sprites: Vec<input_sprite::InputSpritePixelCoord>,
    sprite_table: Vec<SpriteResolution>,
    image: RgbaImage,
}

impl<R: gfx::Resources> SpriteSheetBuilder<R> {
    fn new<F>(image: RgbaImage, input_sprites: Vec<InputSpritePixelCoord>, factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>
    {

        use self::InputSpritePixelCoord::*;

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

        let mut sprite_table = Vec::new();
        for _ in 0..sprite::NUM_SPRITES {
            sprite_table.push(SpriteResolution::default());
        }

        SpriteSheetBuilder {
            render_target_view: rtv,
            shader_resource_view: srv,
            width,
            height,
            input_sprites,
            sprite_table,
            image,
        }
    }

    fn populate(&mut self) {
        for (i, input_sprite) in self.input_sprites.iter().enumerate() {

        }
    }

    fn build(self) -> SpriteSheet<R> {
        let Self { shader_resource_view, width, height, sprite_table, .. } = self;
        SpriteSheet {
            shader_resource_view,
            width,
            height,
            sprite_table,
        }
    }
}

impl<R: gfx::Resources> SpriteSheet<R> {
    pub fn new<F>(image: RgbaImage, input_sprites: Vec<InputSpritePixelCoord>, factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>
    {
        let mut builder = SpriteSheetBuilder::new(image, input_sprites, factory);
        builder.populate();
        builder.build()
    }
}
