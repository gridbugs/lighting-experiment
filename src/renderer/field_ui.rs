use gfx;
use renderer::formats::{ColourFormat, DepthFormat};
use renderer::render_target::RenderTarget;
use renderer::common;
use renderer::sprite_sheet::FieldUiSpriteTable;

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_pipeline!( pipe {
    vertex: gfx::VertexBuffer<Vertex> = (),
    out_colour: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub struct FieldUi<R: gfx::Resources> {
    bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
    sprite_table: FieldUiSpriteTable,
}

impl<R: gfx::Resources> FieldUi<R> {
    pub fn new<F>(sprite_table: FieldUiSpriteTable, target: &RenderTarget<R>, factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {

        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/field_ui.150.vert"),
            include_bytes!("shaders/field_ui.150.frag"),
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

        let data = pipe::Data {
            vertex: vertex_buffer,
            out_colour: target.rtv.clone(),
            out_depth: target.dsv.clone(),
        };

        let bundle = gfx::pso::bundle::Bundle::new(slice, pso, data);

        Self {
            bundle,
            sprite_table,
        }
    }

    pub fn handle_resize<C>(&mut self, target: &RenderTarget<R>, _encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        self.bundle.data.out_colour = target.rtv.clone();
        self.bundle.data.out_depth = target.dsv.clone();
    }

    pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.clear(&self.bundle.data.out_colour, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &self.bundle.data);
    }
}
