pub mod ui;

pub mod task {
    #[derive(Debug, Clone)]
    pub struct Task {
        pub done: bool,
        pub msg: String,
        pub details: Option<String>,
    }

    impl Task {
        pub fn new(msg: String, details: Option<String>) -> Task {
            Task {
                done: false,
                msg,
                details,
            }
        }
    }
}
