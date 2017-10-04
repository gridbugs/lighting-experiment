use gfx;

gfx_constant_struct!( ScrollOffset {
    scroll_offset_pix: [f32; 2] = "u_ScrollOffsetPix",
});

pub type ScrollOffsetBuffer<R> = gfx::handle::Buffer<R, ScrollOffset>;

pub fn create_buffer<R, F>(factory: &mut F) -> ScrollOffsetBuffer<R>
    where R: gfx::Resources,
          F: gfx::Factory<R> + gfx::traits::FactoryExt<R>,
{
    factory.create_constant_buffer(1)
}
