pub trait Geometry {
    fn get_len() -> u32;

    fn vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer;
}