use core::{
    pin::Pin,
    task::{Context, Poll},
};

use alloc::vec::Vec;
use futures_util::{task::AtomicWaker, Stream, StreamExt};
use lazy_static::lazy_static;
use spin::Mutex;
use vga::{
    colors::Color16,
    drawing::Point,
    writers::{Graphics640x480x16, GraphicsWriter, PrimitiveDrawing},
};

lazy_static! {
    pub static ref DRAWING_QUEUE: Mutex<Vec<DrawTask>> = Mutex::default();
    pub static ref DRAWING_CHANGED: Mutex<bool> = Mutex::default();
}

pub fn add_draw_task(task: DrawTask) {
    DRAWING_QUEUE.lock().push(task);
    *DRAWING_CHANGED.lock() = true;
}

pub fn draw_tasks() {
    DRAWING_WAKER.wake();
}

static DRAWING_WAKER: AtomicWaker = AtomicWaker::new();

#[derive(Clone)]
pub enum DrawTask {
    DrawLine {
        x: Point<isize>,
        y: Point<isize>,
        color: Color16,
    },
    DrawText {},
}

pub async fn draw_and_paint() {
    let mut waiter = DrawingWaiter::default();

    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::White);

    while waiter.next().await.is_some() {
        mode.clear_screen(Color16::White);
        let queue = DRAWING_QUEUE.lock().clone();
        for task in queue {
            match task {
                DrawTask::DrawLine { x, y, color } => {
                    mode.draw_line(x, y, color);
                }
                DrawTask::DrawText {} => {}
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
