use crate::listener::listening::{Buffer, Task};

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

impl Task {
    pub fn file_to_task(&mut self) -> io::Result<()> {
        let file_path = "/Users/imadtahri/code/rust/task/save/task.tsk";
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        let reader = BufReader::new(&file);
        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() {
                continue; //check empty lines
            }
            let mut buf = Buffer::new();
            buf.line = line;
            buf.index = index;
            if let Some(last_char) = buf.line.chars().last() {
                if last_char == '1' {
                    buf.is_completed = true;
                } else {
                    buf.is_completed = false;
                }
            }
            buf.line.pop();
            self.buffer.push(buf);
        }
        Ok(())
    }
    pub fn save_task(&mut self) -> io::Result<()> {
        let mut file = File::create("/Users/imadtahri/code/rust/task/save/task.tsk")?;
        for buf in &self.buffer {
            if buf.is_completed {
                writeln!(file, "{} 1", buf.line)?;
            } else {
                writeln!(file, "{} 0", buf.line)?;
            }
        }
        Ok(())
    }
}
