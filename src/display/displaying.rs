use crate::file_system::file::TaskBuf;
use crate::listener::listening::Task;

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use std::io::{self, Write};

impl TaskBuf {
    pub fn display(&self) {
        let mut stdout = io::stdout();
        let mut ite = 0;

        crossterm::terminal::enable_raw_mode().ok(); // ok() is for ignorint return value
        stdout
            .execute(crossterm::terminal::Clear(ClearType::All))
            .ok();
        stdout.execute(cursor::Hide).ok();
        stdout.execute(cursor::MoveTo(0, 0)).ok();
        for (i, task) in self.tasks.iter().enumerate() {
            stdout.execute(cursor::MoveTo(0, ite as u16)).ok();
            println!("---[{}]---", task.name);
            ite += 1;
            for (j, line) in task.buffer.iter().enumerate() {
                stdout.execute(cursor::MoveTo(0, ite as u16)).ok();
                if i == self.sindex.s_index_buf && j == self.sindex.s_index {
                    if !self.tasks[i].buffer[j].is_completed {
                        println!("\x1b[31m>\x1b[34m\t[ ]{}\x1b[0m", line.line);
                    } else {
                        println!("\x1b[31m>\x1b[34m\t[X]{}\x1b[0m", line.line);
                    }
                } else {
                    if !self.tasks[i].buffer[j].is_completed {
                        println!(" \t[ ]{}", line.line);
                    } else {
                        println!(" \t[X]{}", line.line);
                    }
                }
                ite += 1;
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
}
