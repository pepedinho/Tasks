use crate::listener::listening::Task;

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
}
