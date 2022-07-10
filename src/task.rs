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
    
    pub fn new_task(&mut self, msg: String) {
        self.vec.push(Task::new(msg));
    }
}