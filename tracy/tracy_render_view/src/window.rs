use std::sync::Arc;
use winit::{
    dpi::PhysicalSize, event_loop::ActiveEventLoop,
    window::Window as WindowHandler,
};

pub struct Window {
    pub handler: Arc<WindowHandler>,
}

impl Window {
    pub fn new(w: u32, h: u32, event_loop: &ActiveEventLoop) -> Self {
        let window_attrs = WindowHandler::default_attributes()
            .with_inner_size(PhysicalSize::new(w, h))
            .with_resizable(false);

        let handler = event_loop
            .create_window(window_attrs)
            .expect("failed to create window");

        Self {
            handler: Arc::new(handler),
        }
    }
}
