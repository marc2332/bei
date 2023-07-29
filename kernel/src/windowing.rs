use alloc::{format, string::String};
use hashbrown::HashMap;
use vga::{colors::Color16, drawing::Point};

use crate::drawing::{add_draw_task, remove_tasks, DrawTask};

#[derive(PartialEq, Eq, Hash)]
pub struct Window {
    position: Point<isize>,
    size: (usize, usize),
    title: String,
    pub id: usize,
    is_active: bool,
}

impl Window {
    pub fn new(
        id: usize,
        title: String,
        size: (usize, usize),
        is_active: bool,
        position: (isize, isize),
    ) -> Self {
        Self {
            position,
            title,
            size,
            id,
            is_active,
        }
    }

    pub fn draw(&self) {
        remove_tasks(self.id);

        let (color, status) = if self.is_active {
            (Color16::LightCyan, "(Active Window)")
        } else {
            (Color16::LightBlue, "")
        };

        add_draw_task(
            self.id,
            DrawTask::DrawRect {
                start: self.position,
                end: (
                    self.position.0 + self.size.0 as isize,
                    self.position.1 + self.size.1 as isize,
                ),
                color,
            },
        );

        add_draw_task(
            self.id,
            DrawTask::DrawText {
                location: (self.position.0 as usize + 5, self.position.1 as usize + 5),
                text: format!("{} {}", self.title, status),
                color: Color16::Black,
            },
        );
    }
}

pub struct WindowManager {
    pub(crate) windows: HashMap<usize, Window>,
    pub(crate) active_window: Option<usize>,
}

impl WindowManager {
    pub fn new(active_window: Option<usize>, windows: HashMap<usize, Window>) -> Self {
        Self {
            windows,
            active_window,
        }
    }

    pub fn focus_next_window(&mut self) {
        let window = self.windows.iter().enumerate().find_map(|(i, w)| {
            if Some(*w.0) == self.active_window {
                Some(i)
            } else {
                None
            }
        });
        if let Some(window) = window {
            if window == self.windows.len() - 1 {
                self.set_active(self.windows.values().nth(0).map(|w| w.id));
            } else {
                self.set_active(self.windows.values().nth(window + 1).map(|w| w.id));
            }
        }
    }

    pub fn sync_active_window(&mut self, is_active: bool) {
        let window = self
            .windows
            .iter_mut()
            .find(|(id, _)| Some(*id) == self.active_window.as_ref());
        if let Some((_, window)) = window {
            window.is_active = is_active;
            window.draw();
        }
    }

    pub fn set_active(&mut self, active_window: Option<usize>) {
        self.sync_active_window(false);
        self.active_window = active_window;
        self.sync_active_window(true);
    }

    pub fn draw(&self) {
        for window in self.windows.values() {
            window.draw();
        }
    }
}
