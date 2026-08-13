use super::render_item::RenderItem;

pub struct Scene {
    items: Vec<RenderItem>,
}

impl Scene {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: RenderItem) {
        self.items.push(item);
    }

    pub fn render<'a>(&'a self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        for item in &self.items {
            item.render(device, render_pass);
        }
    }
}
