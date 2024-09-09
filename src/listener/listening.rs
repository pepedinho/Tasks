use std::{
    io::{self, Stdout, Write},
    time::Duration,
    usize,
};

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    terminal::{self, ClearType},
    ExecutableCommand,
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
    pub fn display(&self) {
        let mut stdout = io::stdout();

        crossterm::terminal::enable_raw_mode().ok(); // ok() is for ignorint return value
        stdout
            .execute(crossterm::terminal::Clear(ClearType::All))
            .ok();
        stdout.execute(cursor::MoveTo(0, 0)).ok();
        for (i, line) in self.buffer.iter().enumerate() {
            stdout.execute(cursor::MoveTo(0, i as u16)).ok();
            if i == self.s_index {
                if !self.buffer[i].is_completed {
                    println!("\x1b[31m>\x1b[34m [ ]{}\x1b[0m", line.line);
                } else {
                    println!("\x1b[31m>\x1b[34m [X]{}\x1b[0m", line.line);
                }
            } else {
                if !self.buffer[i].is_completed {
                    println!("[ ]{}", line.line);
                } else {
                    println!("[X]{}", line.line);
                }
            }
        }
        stdout.flush().ok();
    }
    pub fn display_popup(&self, line: &String, cas: bool) {
        let mut stdout = io::stdout();
        let (x, y) = terminal::size().unwrap();
        crossterm::terminal::enable_raw_mode().ok();
        if cas {
            stdout
                .execute(crossterm::terminal::Clear(ClearType::All))
                .ok();
        }
        for (i, ch) in line.chars().enumerate() {
            stdout
                .execute(cursor::MoveTo((x / 2) + i as u16, y / 2))
                .ok();
            print!("{}", ch);
        }
        stdout
            .execute(cursor::MoveTo((x / 2) + line.len() as u16, y / 2))
            .ok();
    }
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
                            if self.s_index < self.buffer.len() {
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
}
