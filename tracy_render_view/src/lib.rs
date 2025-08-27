mod render;

use crate::render::RTRenderer;
use std::{
    sync::{Arc, mpsc},
    thread,
};
use tracy_render::{Frame, Renderer};
use tracy_scene::{Geo, Scene};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowId},
};

pub struct RenderView<T>
where
    T: Geo + Sync + Send + 'static,
{
    // FIXME: This organization could be improved.
    rt_renderer: Option<RTRenderer>,
    renderer: Arc<Renderer>,
    scene: Arc<Scene<T>>,
    rx: mpsc::Receiver<Frame>,
    tx: mpsc::Sender<Frame>,
    max_spp: usize,
}

impl<T> RenderView<T>
where
    T: Geo + Sync + Send + 'static,
{
    pub fn new(scene: Scene<T>) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            rt_renderer: None,
            renderer: Arc::new(Renderer { spp: 1, depth: 10 }),
            scene: Arc::new(scene),
            rx,
            tx,
            max_spp: 50,
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().expect("failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(self).expect("failed to run app");
    }
}
impl<T> ApplicationHandler for RenderView<T>
where
    T: Geo + Sync + Send + 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let img_w = self.scene.cam.img_w as u32;
        let img_h = self.scene.cam.img_h as u32;

        // FIXME: Resizing the window distort the texture.
        // FIXME: The output image appears bigger than the same buffer in the
        // real-time render view. (physical size vs logical size)
        let window_attrs = Window::default_attributes()
            .with_inner_size(PhysicalSize::new(img_w, img_h))
            .with_resizable(false);

        let window = event_loop
            .create_window(window_attrs)
            .expect("failed to create window");

        self.rt_renderer =
            Some(pollster::block_on(RTRenderer::new(Arc::new(window))));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let rt_renderer = self.rt_renderer.as_mut().unwrap();

                if let Ok(px_data) = self.rx.try_recv() {
                    println!("spp: {}", rt_renderer.current_spp());
                    rt_renderer.update(&px_data);
                    rt_renderer.render();
                }

                if rt_renderer.current_spp() <= self.max_spp {
                    let window = Arc::clone(&rt_renderer.window);
                    let renderer = Arc::clone(&self.renderer);
                    let scene = Arc::clone(&self.scene);
                    let tx = self.tx.clone();

                    thread::spawn(move || {
                        let frame_bufs = renderer.render(&scene);
                        let frame_buf = frame_bufs.first().unwrap();
                        // FIXME: Remove .clone().
                        let _ = tx.send(frame_buf.frame.clone());
                        window.request_redraw();
                    });
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => match (code, state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            },
            _ => (),
        }
    }
}
