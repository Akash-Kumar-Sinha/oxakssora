use std::{sync::OnceLock, time::Instant};

static START_TIME: OnceLock<Instant> = OnceLock::new();

pub fn bg_cycle(encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
    START_TIME.get_or_init(Instant::now);

    let elapsed = START_TIME.get().unwrap().elapsed().as_secs_f32();

    let t = (elapsed.cos() + 1.0) / 2.0;

    let color = wgpu::Color {
        r: 0.0,
        g: t as f64,
        b: (1.0 - t) as f64,
        a: 1.0,
    };

    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Background Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
}
