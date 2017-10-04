use gfx;

gfx_constant_struct!( FrameInfo {
    frame_count: [u32; 2] = "u_FrameCount_u64",
    total_time_ms: [u32; 2] = "u_TotalTimeMs_u64",
    num_lights: u32 = "u_NumLights",
});

fn u64_to_arr(u: u64) -> [u32; 2] {
    [ u as u32, (u >> 32) as u32 ]
}

impl FrameInfo {
    pub fn update<R, C>(buffer: &FrameInfoBuffer<R>,
                        frame_count: u64,
                        total_time_ms: u64,
                        num_lights: usize,
                        encoder: &mut gfx::Encoder<R, C>)
        where R: gfx::Resources,
              C: gfx::CommandBuffer<R>,
    {
        encoder.update_constant_buffer(buffer, &Self {
            frame_count: u64_to_arr(frame_count),
            total_time_ms: u64_to_arr(total_time_ms),
            num_lights: num_lights as u32,
        });
    }

    pub fn create_buffer<R, F>(factory: &mut F) -> FrameInfoBuffer<R>
        where R: gfx::Resources,
              F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        factory.create_constant_buffer(1)
    }
}

pub type FrameInfoBuffer<R> = gfx::handle::Buffer<R, FrameInfo>;
