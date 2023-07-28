use core::{
    pin::Pin,
    task::{Context, Poll},
};

use alloc::vec::Vec;
use futures_util::{task::AtomicWaker, Stream, StreamExt};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use spin::Mutex;
use vga::{
    colors::Color16,
    drawing::Point,
    writers::{Graphics640x480x16, GraphicsWriter, PrimitiveDrawing},
};

lazy_static! {
    pub static ref DRAWING_QUEUE: Mutex<HashMap<usize, Vec<DrawTask>>> = Mutex::default();
    pub static ref DRAWING_CHANGED: Mutex<bool> = Mutex::default();
}

pub fn add_draw_task(id: usize, task: DrawTask) {
    DRAWING_QUEUE.lock().entry(id).or_default().push(task);
    *DRAWING_CHANGED.lock() = true;
}

pub fn remove_tasks(id: usize) {
    DRAWING_QUEUE.lock().remove(&id);
    *DRAWING_CHANGED.lock() = true;
}

pub fn draw_tasks() {
    DRAWING_WAKER.wake();
}

static DRAWING_WAKER: AtomicWaker = AtomicWaker::new();

#[derive(Clone)]
pub enum DrawTask {
    DrawLine {
        start: Point<isize>,
        end: Point<isize>,
        color: Color16,
    },
    DrawRect {
        start: Point<isize>,
        end: Point<isize>,
        color: Color16,
    },
    DrawText {},
}

pub async fn draw_and_paint() {
    let mut waiter = DrawingWaiter::default();

    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);

    while waiter.next().await.is_some() {
        mode.clear_screen(Color16::Black);
        let queue = DRAWING_QUEUE.lock().clone();
        for tasks in queue.into_values() {
            for task in tasks {
                match task {
                    DrawTask::DrawLine { start, end, color } => {
                        mode.draw_line(start, end, color);
                    }
                    DrawTask::DrawRect { start, end, color } => {
                        for y_i in start.1..end.1 {
                            mode.draw_line((start.0, y_i), (end.0, y_i), color);
                        }
                    }
                    DrawTask::DrawText {} => {}
                }
            }
        }
        *DRAWING_CHANGED.lock() = false;
    }
}

#[derive(Default)]
pub struct DrawingWaiter {
    _private: (),
}

impl Stream for DrawingWaiter {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let queue = DRAWING_QUEUE
            .try_lock()
            .expect("drawing queue not initialized");

        DRAWING_WAKER.register(cx.waker());

        let drawing_changed = *DRAWING_CHANGED.lock();

        if queue.is_empty() || !drawing_changed {
            Poll::Pending
        } else {
            Poll::Ready(Some(()))
        }
    }
}
