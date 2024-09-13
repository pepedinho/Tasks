use file_system::file::TaskBuf;
use listener::listening::Task;

mod display;
mod file_system;
mod listener;
mod save_task;

fn main() {
    let mut task = TaskBuf::new();
    task.listen().ok();
    //task.tasks[0] = Task::new();
    //task.tasks[0].file_to_task().ok();
    //task.file_to_task().ok();
    //task.listen().ok();
    //task.save_task().ok();
}
