use super::listening::{Buffer, Task};
use crate::file_system::file::TaskBuf;

use crossterm::event::{self, KeyCode, KeyEvent};
use std::io::{self};

impl TaskBuf {
    pub(crate) fn up(&mut self) {
        if self.sindex.s_index > 0 {
            self.sindex.s_index -= 1;
        } else if self.sindex.s_index == 0 && self.sindex.s_index_buf > 0 {
            self.sindex.s_index_buf -= 1;
            self.sindex.s_index = self.tasks[self.sindex.s_index_buf].buffer.len() - 1;
        }
    }

    pub(crate) fn down(&mut self) {
        if !self.tasks.is_empty() && !self.tasks[self.sindex.s_index_buf].buffer.is_empty() {
            if self.sindex.s_index < self.tasks[self.sindex.s_index_buf].buffer.len() - 1 {
                self.sindex.s_index += 1;
            } else if self.sindex.s_index == self.tasks[self.sindex.s_index_buf].buffer.len() - 1
                && self.sindex.s_index_buf < self.tasks.len() - 1
            {
                if self.tasks[self.sindex.s_index_buf].buffer.len() != 0 {
                    self.sindex.s_index_buf += 1;
                    self.sindex.s_index = 0;
                }
            }
        }
    }

    pub(crate) fn enter(&mut self) {
        if self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index].is_dir {
            if self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index].is_deploy {
                self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index].is_deploy = false;
            } else {
                self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index].is_deploy = true;
            }
        } else {
            if self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index].is_completed {
                self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index].is_completed =
                    false;
            } else {
                self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index].is_completed = true;
            }
        }
    }

    pub(crate) fn add(&mut self) -> io::Result<()> {
        let mut buf = Buffer::new();
        let mut task = Task::new();
        loop {
            //clear screen and display user input in center of sreen
            self.display_popup(&buf.line);
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char(ca) => {
                        buf.line.push(ca);
                    }
                    KeyCode::Backspace => {
                        buf.line.pop();
                    }
                    KeyCode::Enter => {
                        buf.index = self.sindex.s_index + 1;
                        break;
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        if let Some(last_char) = buf.line.chars().last() {
            if last_char == '/' {
                buf.line.pop();
                buf.is_dir = true;
                buf.is_deploy = true;
                task.buffer.push(buf);
                self.tasks.push(task);
            } else if buf.line.contains('/') {
                let count = buf.line.matches('/').count();
                if count == 1 {
                    let mut split_iter = buf.line.split('/');
                    let mut buf_dir = Buffer::new();
                    let dir = split_iter.next().unwrap();
                    let task_l = split_iter.next().unwrap();
                    buf_dir.line = dir.to_string();
                    buf_dir.is_dir = true;
                    buf_dir.is_deploy = true;
                    task.buffer.push(buf_dir);
                    let mut buf_t = Buffer::new();
                    buf_t.line = task_l.to_string();
                    task.buffer.push(buf_t);
                    self.tasks.push(task);
                } else {
                    let str = "to much directory";
                    self.clean_input();
                    self.display_warning(&str.to_string());
                }
            } else {
                if self.tasks.is_empty() {
                    task.buffer.push(buf);
                    self.tasks.push(task);
                } else {
                    self.tasks[self.sindex.s_index_buf].buffer.push(buf);
                }
            }
        }
        self.clean_input();
        Ok(())
    }
}
