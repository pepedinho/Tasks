use crate::listener::listening::Task;

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use std::io::{self, Write};

impl Task {
    pub fn display(&self) {
        let mut stdout = io::stdout();

        crossterm::terminal::enable_raw_mode().ok(); // ok() is for ignorint return value
        stdout
            .execute(crossterm::terminal::Clear(ClearType::All))
            .ok();
        stdout.execute(cursor::Hide).ok();
        stdout.execute(cursor::MoveTo(0, 0)).ok();
        for (i, line) in self.buffer.iter().enumerate() {
            stdout.execute(cursor::MoveTo(0, i as u16)).ok();
            if i == self.s_index {
                if !self.buffer[i].is_completed {
                    println!("\x1b[31m>\x1b[34m[ ]{}\x1b[0m", line.line);
                } else {
                    println!("\x1b[31m>\x1b[34m[X]{}\x1b[0m", line.line);
                }
            } else {
                if !self.buffer[i].is_completed {
                    println!(" [ ]{}", line.line);
                } else {
                    println!(" [X]{}", line.line);
                }
            }
        }
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
