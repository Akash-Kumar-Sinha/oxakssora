use crate::geometry::{Geometry, Shape, Vertex};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct Triangle;

impl Geometry for Triangle {
    fn get_len() -> u32 {
        Shape::get_len(VERTICES)
    }

    fn vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        Shape::vertex_buffer(device, VERTICES)
    }
}
