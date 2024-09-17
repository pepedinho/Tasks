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
