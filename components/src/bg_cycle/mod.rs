use std::{sync::OnceLock, time::Instant};

use crate::background;

static START_TIME: OnceLock<Instant> = OnceLock::new();

pub fn bg_cycle<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    view: &'a wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    START_TIME.get_or_init(Instant::now);

    let elapsed = START_TIME.get().unwrap().elapsed().as_secs_f32();

    let t = (elapsed.cos() + 1.0) / 2.0;

    let color = wgpu::Color {
        r: 0.0,
        g: t as f64,
        b: (1.0 - t) as f64,
        a: 1.0,
    };

    background::render_screen_background(encoder, view, color)
}
