use components::{geometry::Geometry, rectangle::Rectangle};

#[non_exhaustive]
pub enum RenderItem {
    Rectangle,
}

impl RenderItem {
    pub fn render<'a>(&'a self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        match self {
            Self::Rectangle => {
                let buffer = Rectangle::vertex_buffer(device);

                render_pass.set_vertex_buffer(0, buffer.slice(..));

                render_pass.draw(0..Rectangle::get_len(), 0..1);
            }
        }
    }
}
