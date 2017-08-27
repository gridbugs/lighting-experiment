use gfx;

use renderer::formats::ColourFormat;
use renderer::common;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
    texel: [f32; 2] = "a_Texel",
});

gfx_constant_struct!( Info {
    tex_size: [f32; 2] = "u_TexSize",
    interpolate_threshold_from_centre: f32 = "u_InterpolateThresholdFromCentre",
    interpolate_strip_width: f32 = "u_InterpolateStripWidth",
});

gfx_pipeline!( pipe {
    vertex: gfx::VertexBuffer<Vertex> = (),
    info: gfx::ConstantBuffer<Info> = "Info",
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out_colour: gfx::RenderTarget<ColourFormat> = "Target0",
});

pub struct Scale<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    in_width: u16,
    in_height: u16,
}

impl<R: gfx::Resources> Scale<R> {
    pub fn new<F>(out_rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
                  in_srv: gfx::handle::ShaderResourceView<R, [f32; 4]>,
                  srv_width: u16,
                  srv_height: u16,
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
                    texel: [t[0] * srv_width as f32, t[1] * srv_height as f32],
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
            info: factory.create_constant_buffer(1),
            out_colour: out_rtv,
            tex: (in_srv, sampler),
        };

        Scale {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
            in_width: srv_width,
            in_height: srv_height,
        }
    }

    pub fn init<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        let (out_width, ..) = self.bundle.data.out_colour.get_dimensions();

        // minimum integer scale factor to make the texture size >= the screen size
        let upscale = (out_width / self.in_width) + if out_width % self.in_width == 0 { 0 } else { 1 };

        // Distance in texels along an axis from the centre of a texel to the point
        // at which interpolation begins.
        // (0.5 / upscale) is the width of the interpolation strip around the inside
        // of a texel. Subtract it from 0.5 to get the distance from the centre to
        // the inner edge of the interpolation strip.
        let interpolate_threshold_from_centre = 0.5 - 0.5 / upscale as f32;

        // Width of interpolation strips in texels. For a given pair of adjacent
        // texels, there is a (0.5 / upscale) texel strip along the boundary inside
        // which interpolation will occur.
        let interpolate_strip_width = 1.0 / upscale as f32;

        encoder.update_constant_buffer(&self.bundle.data.info, &Info {
            tex_size: [self.in_width as f32, self.in_height as f32],
            interpolate_threshold_from_centre,
            interpolate_strip_width,
        });
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

    pub fn handle_resize<C, F>(&mut self,
                               out_rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
                               in_srv: gfx::handle::ShaderResourceView<R, [f32; 4]>,
                               srv_width: u16,
                               srv_height: u16,
                               encoder: &mut gfx::Encoder<R, C>,
                               factory: &mut F)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        self.in_width = srv_width;
        self.in_height = srv_height;
        self.bundle.data.out_colour = out_rtv;
        self.bundle.data.tex.0 = in_srv;

        let vertex_data: Vec<Vertex> = izip!(&common::QUAD_VERTICES, &common::QUAD_TEX_COORDS_UPSIDE_DOWN)
            .map(|(v, t)| {
                Vertex {
                    pos: *v,
                    texel: [t[0] * srv_width as f32, t[1] * srv_height as f32],
                }
            }).collect();

        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(
                &vertex_data,
                &common::QUAD_INDICES[..]);

        self.bundle.data.vertex = vertex_buffer;
        self.bundle.slice = slice;

        self.init(encoder);
    }
}
