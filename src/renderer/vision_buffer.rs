use gfx;

use renderer::common;

pub struct VisionBuffer<R: gfx::Resources> {
    pub buffer: gfx::handle::Buffer<R, u8>,
    pub srv: gfx::handle::ShaderResourceView<R, u8>,
}

impl<R: gfx::Resources> VisionBuffer<R> {
    pub fn new<F>(size: usize, factory: &mut F) -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
    {
        let buffer = common::create_transfer_dst_buffer(size, factory)
            .expect("Failed to create vision buffer");

        let srv = factory.view_buffer_as_shader_resource(&buffer)
            .expect("Failed to view vision buffer as shader resource");

        Self {
            buffer,
            srv,
        }
    }
}
