use todo_tui::ui::start_ui;

fn main() {
    start_ui().unwrap();
}

pub mod task {
    #[derive(Debug, Clone)]
    pub struct Task {
        pub done: bool,
        pub msg: String,
    }

    impl Task {
        pub fn new(msg: String) -> Task {
            Task { done: false, msg }
        }
    }
}
