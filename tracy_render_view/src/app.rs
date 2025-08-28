use crate::{
    dispatcher::Dispatcher,
    graphics::{Graphics, Wgpu},
    state::State,
    window::Window,
};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tracy_math::ColorRGBA;
use tracy_render::Renderer;
use tracy_scene::{Geo, Scene};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

// FIXME: Remove hardcoded graphics backend.
// Give the user the ability to select the backend.
pub struct App<T>
where
    T: Geo + Send + Sync + 'static,
{
    window: Option<Window>,
    backend: Option<Wgpu>,
    renderer: Arc<Renderer>,
    state: Arc<Mutex<State<T>>>,
    dispacher: Dispatcher,
}

impl<T> App<T>
where
    T: Geo + Send + Sync + 'static,
{
    pub fn new(desc: AppDesc<T>) -> Self {
        Self {
            window: None,
            backend: None,
            renderer: Arc::new(Renderer { spp: 1, depth: 50 }),
            state: Arc::new(Mutex::new(State::new(desc.scene, desc.spp))),
            dispacher: Dispatcher::new(),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().expect("failed to build event loop");
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(self).expect("failed to run app");
    }

    fn close(&self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
    }

    fn draw(&mut self) {
        if let Some(frame) = self.dispacher.recv() {
            let backend = self.backend.as_mut().unwrap();
            backend.update(&frame);
            backend.render();
        }

        let renderer = Arc::clone(&self.renderer);
        let state = Arc::clone(&self.state);
        let window = Arc::clone(&self.window.as_ref().unwrap().handler);
        let tx = self.dispacher.tx.clone();

        thread::spawn(move || {
            let mut state_guard = state.lock().unwrap();
            println!("current spp: {}", state_guard.current_spp);

            if state_guard.spp == state_guard.current_spp {
                return;
            }

            let frame_bufs = renderer.render(&state_guard.scene);
            let frame_buf = frame_bufs.first().unwrap();
            let frame = &frame_buf.frame;
            let mut new_frame = Vec::with_capacity(frame.len());

            state_guard.current_spp += 1;
            let inv_current_spp = 1.0 / state_guard.current_spp as f64;

            for (dst, src) in state_guard.frame.iter_mut().zip(frame.iter()) {
                dst.r += src.r;
                dst.g += src.g;
                dst.b += src.b;

                new_frame.push(ColorRGBA {
                    r: dst.r * inv_current_spp,
                    g: dst.g * inv_current_spp,
                    b: dst.b * inv_current_spp,
                    a: dst.a,
                });
            }

            tx.send(new_frame).unwrap();
            window.request_redraw();
        });
    }
}

impl<T> ApplicationHandler for App<T>
where
    T: Geo + Send + Sync + 'static,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let state_guard = self.state.lock().unwrap();
        let w = state_guard.scene.cam.img_w as u32;
        let h = state_guard.scene.cam.img_h as u32;
        let window = Window::new(w, h, event_loop);
        self.backend = Some(Wgpu::new(Arc::clone(&window.handler), w, h));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.close(event_loop),
            WindowEvent::RedrawRequested => self.draw(),
            _ => (),
        }
    }
}

pub struct AppDesc<T>
where
    T: Geo,
{
    pub scene: Scene<T>,
    pub spp: usize,
}
