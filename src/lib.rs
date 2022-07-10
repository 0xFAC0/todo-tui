pub mod ui;

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
