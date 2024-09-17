use file_system::file::TaskBuf;

mod display;
mod file_system;
mod listener;
mod save_task;

fn main() {
    let mut task = TaskBuf::new();
    task.listen().ok();
}
