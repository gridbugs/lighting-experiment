use gfx;

pub const QUAD_VERTICES: [[f32; 2]; 4] = [[-1.0, 1.0],
                                          [-1.0, -1.0],
                                          [1.0, -1.0],
                                          [1.0, 1.0]];

pub const QUAD_TEX_COORDS: [[f32; 2]; 4] = [[0.0, 0.0],
                                            [0.0, 1.0],
                                            [1.0, 1.0],
                                            [1.0, 0.0]];

pub const QUAD_TEX_COORDS_UPSIDE_DOWN: [[f32; 2]; 4] = [[0.0, 1.0],
                                                        [0.0, 0.0],
                                                        [1.0, 0.0],
                                                        [1.0, 1.0]];

pub const QUAD_VERTICES_REFL: [[f32; 2]; 4] = QUAD_TEX_COORDS;

pub const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

pub fn create_instance_buffer<R, F, T>(size: usize, factory: &mut F)
    -> Result<gfx::handle::Buffer<R, T>, gfx::buffer::CreationError>
    where R: gfx::Resources,
          F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
{
    factory.create_buffer(size,
                          gfx::buffer::Role::Vertex,
                          gfx::memory::Usage::Data,
                          gfx::TRANSFER_DST)
}

pub fn create_transfer_dst_buffer<R, F, T>(size: usize, factory: &mut F)
    -> Result<gfx::handle::Buffer<R, T>, gfx::buffer::CreationError>
    where R: gfx::Resources,
          F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
{
    factory.create_buffer(size,
                          gfx::buffer::Role::Constant,
                          gfx::memory::Usage::Data,
                          gfx::memory::TRANSFER_DST)
}
