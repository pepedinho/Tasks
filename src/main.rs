use listener::listening::Task;

mod display;
mod listener;
mod save_task;

fn main() {
    let mut task = Task::new();
    task.file_to_task().ok();
    task.listen().ok();
    task.save_task().ok();
}
