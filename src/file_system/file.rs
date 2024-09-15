use crate::listener::listening::{Buffer, Task};
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::ClearType,
    ExecutableCommand,
};
use std::{
    io::{self},
    time::Duration,
};

pub struct Sindex {
    pub s_index: usize,
    pub s_index_buf: usize,
}

pub struct TaskBuf {
    pub tasks: Vec<Task>,
    pub sindex: Sindex,
    pub filename: String,
}

impl Sindex {
    pub fn new() -> Self {
        let s_index = 0;
        let s_index_buf = 0;
        Sindex {
            s_index,
            s_index_buf,
        }
    }
}

impl TaskBuf {
    pub fn new() -> Self {
        let tasks = Vec::new();
        let sindex = Sindex::new();
        let filename = "save/task.tsk".to_string();
        TaskBuf {
            tasks,
            sindex,
            filename,
        }
    }
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
                            if self.sindex.s_index > 0 {
                                self.sindex.s_index -= 1;
                            } else if self.sindex.s_index == 0 && self.sindex.s_index_buf > 0 {
                                self.sindex.s_index_buf -= 1;
                                self.sindex.s_index =
                                    self.tasks[self.sindex.s_index_buf].buffer.len() - 1;
                            }
                        }
                        KeyCode::Down => {
                            if !self.tasks.is_empty()
                                && !self.tasks[self.sindex.s_index_buf].buffer.is_empty()
                            {
                                if self.sindex.s_index
                                    < self.tasks[self.sindex.s_index_buf].buffer.len() - 1
                                {
                                    self.sindex.s_index += 1;
                                } else if self.sindex.s_index
                                    == self.tasks[self.sindex.s_index_buf].buffer.len() - 1
                                    && self.sindex.s_index_buf < self.tasks.len() - 1
                                {
                                    if self.tasks[self.sindex.s_index_buf].buffer.len() != 0 {
                                        self.sindex.s_index_buf += 1;
                                        self.sindex.s_index = 0;
                                    }
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index]
                                .is_dir
                            {
                                if self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index]
                                    .is_deploy
                                {
                                    self.tasks[self.sindex.s_index_buf].buffer
                                        [self.sindex.s_index]
                                        .is_deploy = false;
                                } else {
                                    self.tasks[self.sindex.s_index_buf].buffer
                                        [self.sindex.s_index]
                                        .is_deploy = true;
                                }
                            } else {
                                if self.tasks[self.sindex.s_index_buf].buffer[self.sindex.s_index]
                                    .is_completed
                                {
                                    self.tasks[self.sindex.s_index_buf].buffer
                                        [self.sindex.s_index]
                                        .is_completed = false;
                                } else {
                                    self.tasks[self.sindex.s_index_buf].buffer
                                        [self.sindex.s_index]
                                        .is_completed = true;
                                }
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Char(c) => match c {
                            'a' => {
                                let mut buf = Buffer::new();
                                let mut task = Task::new();
                                loop {
                                    //clear screen and display user input in center of sreen
                                    self.display_popup(&buf.line);
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
