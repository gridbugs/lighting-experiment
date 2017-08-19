use gfx;
use image;
use cgmath::Vector2;

use renderer::tile_renderer::TileRenderer;
use renderer::scale::Scale;
use renderer::formats::ColourFormat;
use renderer::sprite_sheet::SpriteSheet;

use entity_store::EntityStore;
use spatial_hash::SpatialHashTable;

use res::{input_sprite, paths, files};

pub struct Renderer<R: gfx::Resources> {
    tile_renderer: TileRenderer<R>,
    scale: Scale<R>,
}

impl<R: gfx::Resources> Renderer<R> {
    pub fn new<C, F, D>(rtv: &gfx::handle::RenderTargetView<R, ColourFormat>,
                        factory: &mut F,
                        encoder: &mut gfx::Encoder<R, C>,
                        device: &mut D) -> Self
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
              D: gfx::traits::Device<Resources=R, CommandBuffer=C>,
    {
        let sprite_sheet_path = paths::res_path(files::SPRITE_SHEET);
        let image = image::open(&sprite_sheet_path)
            .expect(format!("Failed to open sprite sheet (looked for {})",
                            sprite_sheet_path.display()).as_ref())
            .to_rgba();
        let sprite_sheet =
            SpriteSheet::new(image, input_sprite::input_sprites(),
                             factory, encoder, device);

        let (width_px, height_px, ..) = rtv.get_dimensions();

        let (tile_renderer, srv) = TileRenderer::new(sprite_sheet, width_px, height_px, factory);
        let scale = Scale::new(rtv.clone(), srv, factory);

        tile_renderer.init(encoder);

        Renderer {
            tile_renderer,
            scale,
        }
    }

    pub fn update_offset<C>(&self, player_position: Vector2<f32>, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.tile_renderer.update_offset(player_position, encoder);
    }

    pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.tile_renderer.clear(encoder);
        self.scale.clear(encoder);
    }

    pub fn render<C, F>(&mut self,
                        entity_store: &EntityStore,
                        spatial_hash: &SpatialHashTable,
                        factory: &mut F,
                        encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        self.tile_renderer.update_entities(entity_store, spatial_hash, factory);
        self.tile_renderer.draw(encoder);
        self.scale.draw(encoder);
    }
}
