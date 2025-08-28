use crate::graphics::Frame;
use mpsc::{Receiver, Sender};
use std::sync::mpsc;

pub struct Dispatcher {
    rx: Receiver<Frame>,
    pub tx: Sender<Frame>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { rx, tx }
    }

    pub fn recv(&self) -> Option<Frame> {
        if let Ok(frame) = self.rx.try_recv() {
            return Some(frame);
        }

        None
    }
}
