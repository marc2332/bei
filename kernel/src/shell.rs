use alloc::string::String;
use pc_keyboard::KeyCode;

use crate::{
    print, println,
    vga::{_clear, _clear_all},
};

#[derive(Default)]
pub struct Shell {
    input: String,
}

impl Shell {
    pub fn new() -> Self {
        let mut this = Self::default();
        this.new_line();
        this
    }

    fn run_command(&mut self) {
        match self.input.as_str() {
            "clear" => {
                _clear_all();
                self.input.clear();
            }
            "help" => {
                println!("  clear: Clear the output");
                println!("  help: Show help info");
            }
            _ => {}
        }
    }

    fn render_prompt(&self) {
        print!("> ");
    }

    fn new_line(&mut self) {
        self.input.clear();
        self.render_prompt();
    }

    fn press_enter(&mut self) {
        println!("");
        if !self.input.is_empty() {
            self.run_command();
        }
        self.new_line();
    }

    fn remove_last_row(&mut self) {
        _clear();
    }

    pub fn press_char(&mut self, ch: char) {
        match ch as u8 {
            10 => {
                self.press_enter();
            }
            8 => {
                if !self.input.is_empty() {
                    self.remove_last_row();
                    self.input.remove(self.input.len() - 1);
                    self.render_prompt();
                    print!("{}", self.input);
                }
            }
            _ => {
                self.input.push(ch);
                print!("{ch}");
            }
        }
    }

    pub fn press_rawkey(&mut self, _code: KeyCode) {}
}
