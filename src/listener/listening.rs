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
    pub name: String,
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
        let name = String::new();
        Task {
            buffer,
            s_index,
            name,
        }
    }
}
