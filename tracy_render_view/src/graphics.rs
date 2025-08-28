pub mod wgpu;

pub use wgpu::Wgpu;

use std::sync::Arc;
use tracy_math::ColorRGBA;
use winit::window::Window as WindowHandle;

pub type Frame = Vec<ColorRGBA<f64>>;

pub trait Graphics {
    fn new(window_handler: Arc<WindowHandle>, tex_w: u32, tex_h: u32) -> Self;
    fn update(&mut self, buf: &Frame);
    fn render(&self);
}
