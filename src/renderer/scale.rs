use gfx;

use renderer::formats::ColourFormat;
use renderer::common;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
    tex: [f32; 2] = "a_Tex",
});

gfx_pipeline!( pipe {
    vertex: gfx::VertexBuffer<Vertex> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out_colour: gfx::RenderTarget<ColourFormat> = "Target0",
});

pub struct Scale<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> Scale<R> {
    pub fn new<F>(out_rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
                  in_srv: gfx::handle::ShaderResourceView<R, [f32; 4]>,
                  factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/shdr_150_scale.vert"),
            include_bytes!("shaders/shdr_150_scale.frag"),
            pipe::new()).expect("Failed to create pipeline");

        let vertex_data: Vec<Vertex> = izip!(&common::QUAD_VERTICES, &common::QUAD_TEX_COORDS_UPSIDE_DOWN)
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

        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale,
                                           gfx::texture::WrapMode::Tile));

        let data = pipe::Data {
            vertex: vertex_buffer,
            out_colour: out_rtv,
            tex: (in_srv, sampler),
        };

        Scale {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
        }
    }

    pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.clear(&self.bundle.data.out_colour, [0.0, 0.0, 0.0, 1.0]);
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }
}
