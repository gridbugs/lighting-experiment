use gfx;

use renderer::sprite_sheet::SpriteSheet;
use renderer::formats::ColourFormat;
use renderer::common;

const MAX_NUM_INSTANCES: usize = 4096;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_vertex_struct!( Instance {
    sprite_index: f32 = "a_SpriteIndex",
    size: [f32; 2] = "a_SizePix",
    coord: [f32; 2] = "a_CoordPix",
});

gfx_constant_struct!( Locals {
    output_size: [f32; 2] = "u_OutputSizePix",
    sprite_size: [f32; 2] = "u_SpriteSize",
});

gfx_pipeline!( pipe {
    locals: gfx::ConstantBuffer<Locals> = "Locals",
    vertex: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    out: gfx::RenderTarget<ColourFormat> = "Target0",
});

pub struct TileRenderer<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    upload: gfx::handle::Buffer<R, Instance>,
    sprite_sheet: SpriteSheet<R>,
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

        let vertex_data: Vec<Vertex> = common::QUAD_VERTICES.iter()
            .map(|v| {
                Vertex {
                    pos: *v,
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
            locals: factory.create_constant_buffer(1),
            vertex: vertex_buffer,
            instance: common::create_instance_buffer(MAX_NUM_INSTANCES, factory)
                .expect("Failed to create instance buffer"),
            out: rtv,
            tex: (sprite_sheet.shader_resource_view.clone(), sampler),
        };

        TileRenderer {
            bundle: gfx::pso::bundle::Bundle::new(slice, pso, data),
            upload: factory.create_upload_buffer(MAX_NUM_INSTANCES)
                .expect("Failed to create upload buffer"),
            sprite_sheet,
        }
    }

    pub fn init<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {

        let (width, height, ..) = self.bundle.data.out.get_dimensions();

        let sprite_size = self.sprite_sheet.sprite_size();
        encoder.update_constant_buffer(&self.bundle.data.locals, &Locals {
            output_size: [width as f32, height as f32],
            sprite_size: [sprite_size.0, sprite_size.1],
        });
    }

    pub fn update<C, F>(&self, encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        {
            let mut mapper = factory.write_mapping(&self.upload)
                .expect("Failed to map upload buffer");

            mapper[0] = Instance {
                sprite_index: 1.0,
                size: [64.0, 64.0],
                coord: [32.0, 8.0],
            };
        }

        encoder.copy_buffer(&self.upload, &self.bundle.data.instance, 0, 0, 1)
            .expect("Failed to copy instances");
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }
}
