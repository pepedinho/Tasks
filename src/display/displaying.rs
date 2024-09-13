use crate::file_system::file::TaskBuf;
use crate::listener::listening::Task;

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use std::io::{self, Write};

impl TaskBuf {
    pub fn clean_board(&self) {
        let mut stdout = io::stdout();
        let mut ite = 0;

        for (_i, task) in self.tasks.iter().enumerate() {
            for _j in task.buffer.iter().enumerate() {
                stdout.execute(cursor::MoveTo(0, ite as u16)).ok();
                stdout
                    .execute(crossterm::terminal::Clear(ClearType::CurrentLine))
                    .ok();
                ite += 1;
            }
        }
    }
    pub fn display(&self) {
        let mut stdout = io::stdout();
        let mut ite = 0;

        self.clean_board();
        crossterm::terminal::enable_raw_mode().ok(); // ok() is for ignorint return value
        stdout.execute(cursor::Hide).ok();
        stdout.execute(cursor::MoveTo(0, 0)).ok();
        for (i, task) in self.tasks.iter().enumerate() {
            let mut show = false;
            for (j, line) in task.buffer.iter().enumerate() {
                stdout.execute(cursor::MoveTo(0, ite as u16)).ok();
                if task.buffer[0].is_deploy {
                    show = true;
                }
                if line.is_dir && self.is_selected_index(i, j) {
                    println!(
                        "\x1b[31m>\x1b[34m {} {}\x1b[0m",
                        if show { '⌄' } else { '>' },
                        line.line
                    );
                    if !show {
                        ite += 1;
                    }
                } else if line.is_dir {
                    println!("  {} {}", if show { '⌄' } else { '>' }, line.line);
                    if !show {
                        ite += 1;
                    }
                } else if self.is_selected_index(i, j) && show {
                    if !self.tasks[i].buffer[j].is_completed {
                        if j < self.tasks[i].buffer.len() - 1 {
                            println!("\x1b[31m>\x1b[34m   ├─[ ]{}\x1b[0m", line.line);
                        } else {
                            println!("\x1b[31m>\x1b[34m   └─[ ]{}\x1b[0m", line.line);
                        }
                    } else if show {
                        if j < self.tasks[i].buffer.len() - 1 {
                            println!("\x1b[31m>\x1b[34m   ├─[X]{}\x1b[0m", line.line);
                        } else {
                            println!("\x1b[31m>\x1b[34m   └─[X]{}\x1b[0m", line.line);
                        }
                    }
                } else if show {
                    if !self.tasks[i].buffer[j].is_completed {
                        if j < self.tasks[i].buffer.len() - 1 {
                            println!("    ├─[ ]{}", line.line);
                        } else {
                            println!("    └─[ ]{}", line.line);
                        }
                    } else {
                        if j < self.tasks[i].buffer.len() - 1 {
                            println!("    ├─[X]{}", line.line);
                        } else {
                            println!("    └─[X]{}", line.line);
                        }
                    }
                }
                if show {
                    ite += 1;
                }
            }
        }
        stdout.execute(cursor::MoveTo(0, 30)).ok();
        println!("s_index : {:?}", self.sindex.s_index);
        stdout.execute(cursor::MoveTo(0, 31)).ok();
        println!("s_index_buf : {:?}", self.sindex.s_index_buf);
        stdout.execute(cursor::MoveTo(0, 32)).ok();
        println!("nb buffer : {:?}", self.tasks.len());
        stdout.flush().ok();
    }
    pub fn display_popup(&self, line: &String) {
        let mut stdout = io::stdout();
        let (_x, y) = terminal::size().unwrap();
        crossterm::terminal::enable_raw_mode().ok();
        stdout.execute(cursor::MoveTo(0 as u16, y - 1)).ok();
        stdout
            .execute(crossterm::terminal::Clear(ClearType::CurrentLine))
            .ok();
        for (i, ch) in line.chars().enumerate() {
            stdout.execute(cursor::MoveTo(i as u16, y - 1)).ok();
            print!("{}", ch);
        }
        stdout
            .execute(cursor::MoveTo(line.len() as u16, y - 1))
            .ok();
        stdout.execute(cursor::Show).ok();
    }
    pub fn clean_input(&self) {
        let mut stdout = io::stdout();
        stdout
            .execute(crossterm::terminal::Clear(ClearType::CurrentLine))
            .ok();
    }
    pub fn show_cursor(&self) {
        let mut stdout = io::stdout();
        stdout.execute(cursor::Show).ok();
    }
    pub fn is_selected_index(&self, i: usize, j: usize) -> bool {
        if i == self.sindex.s_index_buf && j == self.sindex.s_index {
            return true;
        }
        false
    }
}
