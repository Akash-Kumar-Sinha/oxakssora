use crate::geometry::Vertex;

use wgpu::util::DeviceExt;

pub struct Shape;

impl Shape {
    pub fn get_len(vertices: &[Vertex]) -> u32 {
        vertices.len() as u32
    }

    pub fn vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}
