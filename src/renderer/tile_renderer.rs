use gfx;

use renderer::sprite_sheet::SpriteSheet;
use renderer::formats::ColourFormat;
use renderer::common;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
    tex: [f32; 2] = "a_Tex",
});

gfx_pipeline!( pipe {
    vertex: gfx::VertexBuffer<Vertex> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out: gfx::RenderTarget<ColourFormat> = "Target0",
});

pub struct TileRenderer<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> TileRenderer<R> {
    pub fn new<F>(sprite_sheet: SpriteSheet<R>,
                  rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
                  factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/shdr_330_tile_renderer.vert"),
            include_bytes!("shaders/shdr_330_tile_renderer.frag"),
            pipe::new()).expect("Failed to create pipeline");

        let vertex_data: Vec<Vertex> =
            izip!(&common::QUAD_VERTICES, &common::QUAD_TEX_COORDS)
            .map(|(v, t)| {
                Vertex {
                    pos: *v,
                    tex: *t,
                }
            }).collect();

        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(
                &vertex_data,
                &common::QUAD_INDICES[..]);

        let SpriteSheet { shader_resource_view, .. } = sprite_sheet;
        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale,
                                           gfx::texture::WrapMode::Tile));

        let data = pipe::Data {
            vertex: vertex_buffer,
            out: rtv,
            tex: (shader_resource_view, sampler),
        };

        TileRenderer {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
        }
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }
}
