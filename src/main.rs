use todo_tui::ui::start_ui;

fn main() {
    start_ui().unwrap();
}

#[derive(Debug, Clone)]
pub struct Task {
    pub done: bool,
    pub msg: String,
}

pub struct TaskList {
    pub vec: Vec<Task>,
}

impl Task {
    pub fn new(msg: String) -> Task {
        Task { done: false, msg }
    }
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { vec: vec![] }
    }
}
