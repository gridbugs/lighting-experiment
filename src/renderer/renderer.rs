use gfx;
use image;

use renderer::tile_renderer::{TileRenderer, RendererWorldState};
use renderer::scale::Scale;
use renderer::field_ui::FieldUi;
use renderer::formats::ColourFormat;
use renderer::sprite_sheet;
use renderer::render_target::RenderTarget;
use renderer::dimensions::Dimensions;
use renderer::vision_buffer::VisionBuffer;
use renderer::frame_info::FrameInfo;
use renderer::sizes;
use renderer::scroll_offset;

use res::{input_sprite, paths, files};

use entity_store::EntityStore;

pub struct Renderer<R: gfx::Resources> {
    target: RenderTarget<R>,
    tile_renderer: TileRenderer<R>,
    field_ui: FieldUi<R>,
    scale: Scale<R>,
    dimensions: Dimensions<R>,
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
        let (sprite_sheet, tile_table, field_ui_table) =
            sprite_sheet::create(image, input_sprite::input_sprites(),
                                 factory, encoder, device);

        let (width, height, ..) = rtv.get_dimensions();

        let target = RenderTarget::new((width, height), factory);

        let dimensions = Dimensions::new(factory);
        dimensions.update_fixed_dimensions(&sprite_sheet, encoder);
        dimensions.update_output_dimensions(&target, encoder);

        let vision_buffer = VisionBuffer::new(sizes::LIGHT_BUFFER_SIZE, factory);
        let frame_info_buffer = FrameInfo::create_buffer(factory);
        let scroll_offset_buffer = scroll_offset::create_buffer(factory);

        let tile_renderer = TileRenderer::new(&sprite_sheet,
                                              tile_table,
                                              &target,
                                              &dimensions,
                                              &vision_buffer,
                                              &frame_info_buffer,
                                              &scroll_offset_buffer,
                                              factory);

        let field_ui = FieldUi::new(&sprite_sheet,
                                    field_ui_table,
                                    &target,
                                    &dimensions,
                                    &vision_buffer,
                                    &frame_info_buffer,
                                    &scroll_offset_buffer,
                                    factory);

        let scale = Scale::new(rtv.clone(), target.srv.clone(), target.width, target.height, factory, encoder);

        Renderer {
            target,
            tile_renderer,
            field_ui,
            scale,
            dimensions,
        }
    }

    pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.tile_renderer.clear(encoder);
        self.field_ui.clear(encoder);
        self.scale.clear(encoder);
    }

    pub fn render<C, F>(&mut self, entity_store: &EntityStore, encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        self.tile_renderer.draw(encoder);
        self.field_ui.draw(entity_store, encoder, factory);
        self.scale.draw(encoder);
    }

    pub fn world_state<F>(&mut self, factory: &mut F) -> RendererWorldState<R>
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        self.tile_renderer.world_state(&self.target, factory)
    }

    pub fn handle_resize<C, F>(&mut self, rtv: &gfx::handle::RenderTargetView<R, ColourFormat>,
                               encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let (width, height, ..) = rtv.get_dimensions();
        self.target = RenderTarget::new((width, height), factory);
        self.dimensions.update_output_dimensions(&self.target, encoder);
        self.tile_renderer.handle_resize(&self.target, encoder);
        self.field_ui.handle_resize(&self.target, encoder);
        self.scale.handle_resize(rtv.clone(), self.target.srv.clone(), self.target.width, self.target.height, encoder, factory);
    }

    pub fn update_world_size<C>(&mut self, width: u32, height: u32,
                                encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.dimensions.update_world_dimensions((width, height), encoder);
        self.tile_renderer.update_world_size(width, height);
    }
}
