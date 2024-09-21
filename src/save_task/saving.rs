use crate::file_system::file::TaskBuf;
use crate::listener::listening::{Buffer, Task};

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

impl TaskBuf {
    pub fn file_to_task(&mut self) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.filename)?;
        let reader = BufReader::new(&file);
        let mut b_index = 0;
        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            let indicator: String = line
                .chars()
                .rev()
                .take(2)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            if line.trim().is_empty() {
                continue; //check empty lines
            }
            let mut buf = Buffer::new();
            if indicator == "--" {
                let mut task = Task::new();
                buf.line = line;
                buf.line.pop();
                buf.line.pop();
                buf.is_deploy = true;
                buf.is_dir = true;
                task.buffer.push(buf);
                self.tasks.push(task);
                b_index += 1;
            } else {
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
                if self.tasks.is_empty() {
                    let mut task = Task::new();
                    task.buffer.push(buf);
                    self.tasks.push(task);
                } else {
                    self.tasks[b_index - 1].buffer.push(buf);
                }
            }
        }
        Ok(())
    }
    pub fn save_task(&mut self) -> io::Result<()> {
        let mut file = File::create(&self.filename)?;
        for (_i, task) in self.tasks.iter().enumerate() {
            for (_j, line) in task.buffer.iter().enumerate() {
                if line.is_dir {
                    writeln!(file, "{} --", line.line)?;
                } else if line.is_completed {
                    writeln!(file, "{} 1", line.line)?;
                } else {
                    writeln!(file, "{} 0", line.line)?;
                }
            }
        }
        Ok(())
    }
    pub fn search_tsk_file(&mut self) {
        // serche files with ".tsk" extension in current directory
        let paths = std::fs::read_dir(".").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".tsk") {
                self.filename = file_name.to_string();
                return;
            }
        }
        self.filename = "tasks.tsk".to_string();
    }
}
