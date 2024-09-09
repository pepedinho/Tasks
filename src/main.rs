use listener::listening::Task;

mod listener;

fn main() {
    let mut task = Task::new();
    task.listen().ok();
}
