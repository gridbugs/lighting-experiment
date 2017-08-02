use std::collections::VecDeque;

use gfx;

use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};

use renderer::formats::ColourFormat;

const NUM_INSTANCE_ROWS: u32 = 256;
const NUM_INSTANCE_COLS: u32 = 256;
const NUM_INSTANCES: u32 = NUM_INSTANCE_COLS * NUM_INSTANCE_ROWS;
const INSTANCE_RING_BUFFER_SIZE: usize = 8;
const PADDING: f32 = 0.01;
const STEP_X: f32 = 1.0 / (NUM_INSTANCE_COLS as f32);
const STEP_Y: f32 = 1.0 / (NUM_INSTANCE_ROWS as f32);

gfx_vertex_struct!( TileMapVertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_vertex_struct!( TileMapInstance {
    col: [f32; 4] = "a_Col",
    translate: [f32; 2] = "a_Translate",
});

gfx_pipeline!( example {
    vertex: gfx::VertexBuffer<TileMapVertex> = (),
    instance: gfx::InstanceBuffer<TileMapInstance> = (),
    out_colour: gfx::RenderTarget<ColourFormat> = "Target0",
});

pub struct Example<R: gfx::Resources> {
    slice: gfx::Slice<R>,
    pso: gfx::pso::PipelineState<R, example::Meta>,
    data: example::Data<R>,
    upload: gfx::handle::Buffer<R, TileMapInstance>,
    instance_data: Vec<TileMapInstance>,
    instance_ring: VecDeque<gfx::handle::Buffer<R, TileMapInstance>>,
}

impl<R: gfx::Resources> Example<R> {
    pub fn new<F>(rtv: gfx::handle::RenderTargetView<R, ColourFormat>,
                     factory: &mut F)
        -> Result<Self, gfx::PipelineStateError<String>>
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/shdr_330_example.vert"),
            include_bytes!("shaders/shdr_330_example.frag"),
            example::new()
        ).expect("Failed to create pipeline");

        let plane = Plane::subdivide(1, 1);

        let vertex_data: Vec<TileMapVertex> = plane.shared_vertex_iter().map(|vertex| {
                let x = vertex.pos[0] * 0.5 + 0.5;
                let y = 0.5 - vertex.pos[1] * 0.5;

                TileMapVertex {
                    pos: [ x * (STEP_X - PADDING), y * (STEP_Y - PADDING) ],
                }
            })
            .collect();

        let index_data: Vec<u32> = plane.indexed_polygon_iter()
            .triangulate()
            .vertices()
            .map(|i| i as u32)
            .collect();

        let (vertex_buffer, mut slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
        slice.instances = Some((NUM_INSTANCES, 0));

        let mut instance_ring = VecDeque::new();

        for _ in 0..INSTANCE_RING_BUFFER_SIZE {
            let instance_buffer = factory.create_buffer(NUM_INSTANCES as usize,
                                                        gfx::buffer::Role::Vertex,
                                                        gfx::memory::Usage::Data,
                                                        gfx::TRANSFER_DST).expect("Failed to create instance buffer");

            instance_ring.push_back(instance_buffer);
        }

        let upload = factory.create_upload_buffer(NUM_INSTANCES as usize).expect("Failed to create upload buffer");

        let data = example::Data {
            vertex: vertex_buffer,
            instance: instance_ring.pop_front().unwrap(),
            out_colour: rtv,
        };

        let mut instance_data = Vec::new();
        for _ in 0..NUM_INSTANCES {
            instance_data.push(TileMapInstance {
                col: [0.0, 0.0, 0.0, 0.0],
                translate: [0.0, 0.0],
            });
        }

        for (idx, element) in instance_data.iter_mut().enumerate() {
            let intensity = (idx as f32) / (NUM_INSTANCES as f32);
            let x = idx as u32 % NUM_INSTANCE_COLS;
            let y = idx as u32 / NUM_INSTANCE_COLS;

            let x_trans = x as f32 * STEP_X;
            let y_trans = y as f32 * STEP_Y;

            *element = TileMapInstance {
                col: [intensity, intensity, intensity, 1.0],
                translate: [ x_trans, y_trans ],
            };
        }

        Ok(Example {
            slice: slice,
            pso: pso,
            data: data,
            upload: upload,
            instance_data: instance_data,
            instance_ring: instance_ring,
        })
    }

    pub fn update<C, F>(&mut self, encoder: &mut gfx::Encoder<R, C>, factory: &mut F)
        where C: gfx::CommandBuffer<R>,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        {
            let mut mapping = factory.write_mapping(&self.upload).expect("Failed to map upload buffer");
            for (src, dst) in izip!(self.instance_data.iter(), mapping.iter_mut()) {
                *dst = *src;
            }
        }
        encoder.copy_buffer(&self.upload, &self.data.instance, 0, 0, NUM_INSTANCES as usize).expect("Failed to copy buffer");
    }

    pub fn draw<C>(&self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>,
    {
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
