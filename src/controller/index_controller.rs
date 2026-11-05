use crate::domain::task::Task;

pub async fn root() -> &'static str {
    let first = Task {
        id: 1,
        name: "first".to_string(),
        done: false,
    };
    println!("Task: {}", first.to_string());
    println!("Hello, world!");

    "Server is alive"
}
