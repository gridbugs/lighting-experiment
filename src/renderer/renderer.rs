use gfx;
use image;

use renderer::tile_renderer::{TileRenderer, RendererWorldState};
use renderer::scale::Scale;
use renderer::formats::ColourFormat;
use renderer::sprite_sheet::SpriteSheet;

use res::{input_sprite, paths, files};
use frontend::VisibleRange;

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

        let (srv_width, srv_height) = tile_renderer.dimensions();
        let scale = Scale::new(rtv.clone(), srv, srv_width, srv_height, factory);

        tile_renderer.init(encoder);
        scale.init(encoder);

        Renderer {
            tile_renderer,
            scale,
        }
    }

    pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.tile_renderer.clear(encoder);
        self.scale.clear(encoder);
    }

    pub fn render<C>(&mut self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.tile_renderer.draw(encoder);
        self.scale.draw(encoder);
    }

    pub fn world_state<F>(&mut self, factory: &mut F) -> RendererWorldState<R>
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        self.tile_renderer.world_state(factory)
    }

    pub fn handle_resize<C, F>(&mut self, rtv: &gfx::handle::RenderTargetView<R, ColourFormat>,
                               encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let (width, height, ..) = rtv.get_dimensions();
        let srv = self.tile_renderer.handle_resize(width, height, encoder, factory);
        let (srv_width, srv_height) = self.tile_renderer.dimensions();
        self.scale.handle_resize(rtv.clone(), srv, srv_width, srv_height, encoder, factory);
    }

    pub fn update_world_size<C>(&mut self, width: u32, height: u32,
                                encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.tile_renderer.update_world_size(width, height, encoder);
    }

    pub fn visible_range(&self) -> VisibleRange {
        self.tile_renderer.visible_range()
    }
}
