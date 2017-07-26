use gfx;

use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};

use renderer::formats::ColourFormat;

gfx_vertex_struct!( TileMapVertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_pipeline!( tile_map {
    vbuf: gfx::VertexBuffer<TileMapVertex> = (),
    out_colour: gfx::RenderTarget<ColourFormat> = "Target0",
});

pub fn init<R, F>(rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
                  factory: &mut F)
    -> Result<gfx::Bundle<R, tile_map::Data<R>>, gfx::PipelineStateError<String>>
    where R: gfx::Resources,
          F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
{

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/shdr_330_general.vert"),
        include_bytes!("shaders/shdr_330_red.frag"),
        tile_map::new()
    )?;

    let plane = Plane::subdivide(1, 1);

    let vertex_data: Vec<TileMapVertex> = plane.shared_vertex_iter().map(|vertex| {
            TileMapVertex {
                pos: [ vertex.pos[0], vertex.pos[1] ],
            }
        })
        .collect();

    let index_data: Vec<u32> = plane.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| i as u32)
        .collect();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

    let data = tile_map::Data {
        vbuf: vertex_buffer,
        out_colour: rtv,
    };

    Ok(gfx::Bundle::new(slice, pso, data))
}
