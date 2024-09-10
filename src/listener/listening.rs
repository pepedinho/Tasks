use crossterm::event::{self, KeyCode, KeyEvent};
use std::{
    io::{self},
    time::Duration,
    usize,
};

pub struct Buffer {
    pub line: String,
    pub index: usize,
    pub is_selected: bool,
    pub is_completed: bool,
}

pub struct Task {
    pub buffer: Vec<Buffer>,
    pub s_index: usize,
}

impl Buffer {
    pub fn new() -> Self {
        let line = String::new();
        let index = 0;
        let is_selected = false;
        let is_completed = false;
        Buffer {
            line,
            index,
            is_selected,
            is_completed,
        }
    }
}

impl Task {
    pub fn new() -> Self {
        let buffer = Vec::new();
        let s_index = 0;
        Task { buffer, s_index }
    }
    //listen y/n user input
    pub fn listen_delete(&mut self) {
        let str = "are you sure to delelete this task ?(y/n)";
        self.display_popup(&str.to_string(), false);
        loop {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char(c) => match c {
                        'y' => {
                            self.buffer.remove(self.s_index);
                            break;
                        }
                        'n' => {
                            break;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    //listen all user input
    pub fn listen(&mut self) -> io::Result<()> {
        loop {
            self.display();
            if event::poll(Duration::from_millis(100))? {
                if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        KeyCode::Up => {
                            if self.s_index > 0 {
                                self.s_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.s_index < self.buffer.len() - 1 {
                                self.s_index += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if self.buffer[self.s_index].is_completed {
                                self.buffer[self.s_index].is_completed = false;
                            } else {
                                self.buffer[self.s_index].is_completed = true;
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Char(c) => match c {
                            'a' => {
                                let mut buf = Buffer::new();
                                loop {
                                    //clear screen and display user input in center of sreen
                                    self.display_popup(&buf.line, true);
                                    if let event::Event::Key(KeyEvent { code, .. }) = event::read()?
                                    {
                                        match code {
                                            KeyCode::Char(ca) => {
                                                buf.line.push(ca);
                                            }
                                            KeyCode::Backspace => {
                                                buf.line.pop();
                                            }
                                            KeyCode::Enter => {
                                                buf.index = self.s_index + 1;
                                            }
                                            KeyCode::Esc => {
                                                break;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                self.buffer.push(buf);
                            }
                            'd' => {
                                self.listen_delete();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
        crossterm::terminal::disable_raw_mode().ok();
        Ok(())
    }
    pub fn save_task(&mut self) {
        //TODO: save task in a history file
    }
}
