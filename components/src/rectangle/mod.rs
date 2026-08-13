use crate::geometry::{Geometry, Shape, Vertex};

const VERTICES: &[Vertex] = &[
    // Top-left
    Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    // Bottom-left
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    // Top-right
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    // Top-right
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    // Bottom-left
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    // Bottom-right
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];

pub struct Rectangle;

impl Geometry for Rectangle {
    fn get_len() -> u32 {
        Shape::get_len(VERTICES)
    }

    fn vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        Shape::vertex_buffer(device, VERTICES)
    }
}
