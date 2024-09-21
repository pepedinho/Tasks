use crate::file_system::file::TaskBuf;
use crate::listener::utils;

use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::ClearType,
    ExecutableCommand,
};
use std::{
    io::{self},
    time::Duration,
};

pub struct Buffer {
    pub line: String,
    pub index: usize,
    pub is_completed: bool,
    pub is_dir: bool,
    pub is_deploy: bool,
}

pub struct Task {
    pub buffer: Vec<Buffer>,
}

impl Buffer {
    pub fn new() -> Self {
        let line = String::new();
        let index = 0;
        let is_completed = false;
        let is_dir = false;
        let is_deploy = true;
        Buffer {
            line,
            index,
            is_completed,
            is_dir,
            is_deploy,
        }
    }
}

impl Task {
    pub fn new() -> Self {
        let buffer = Vec::new();
        Task { buffer }
    }
}

impl TaskBuf {
    pub fn listen(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        stdout
            .execute(crossterm::terminal::Clear(ClearType::All))
            .ok();
        self.search_tsk_file();
        self.file_to_task().ok();
        loop {
            self.display();
            if event::poll(Duration::from_millis(100))? {
                if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        KeyCode::Up => {
                            self.up();
                        }
                        KeyCode::Down => {
                            self.down();
                        }
                        KeyCode::Enter => {
                            self.enter();
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Char(c) => match c {
                            'a' => {
                                self.add()?;
                            }
                            'd' => {
                                self.listen_delete();
                                self.clean_input();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
        self.show_cursor();
        crossterm::terminal::disable_raw_mode().ok();
        self.save_task().ok();
        Ok(())
    }

    pub fn listen_delete(&mut self) {
        let str = "are you sure to delelete this task ?(y/n)";
        self.display_popup(&str.to_string());
        loop {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char(c) => match c {
                        'y' => {
                            if self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index]
                                .is_dir
                            {
                                self.tasks.remove(self.sindex.s_index_buf);
                                if self.sindex.s_index_buf > self.tasks.len() - 1 {
                                    self.sindex.s_index_buf -= 1;
                                }
                            } else {
                                self.tasks[self.sindex.s_index_buf]
                                    .buffer
                                    .remove(self.sindex.s_index);
                            }
                            if self.sindex.s_index > 0 {
                                self.sindex.s_index -= 1;
                            } else {
                                self.sindex.s_index += 1;
                            }
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
        self.clean_all();
    }
}
