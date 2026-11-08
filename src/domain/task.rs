use serde::{Deserialize, Serialize};
use std::format;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct TaskCreate {
    pub name: String,
    pub done: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct TaskUpdate {
    pub task_id: Uuid,
    pub name: String,
    pub done: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Task {
    pub task_id: Uuid,
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn to_string(self) -> String {
        format!(
            "Task[ task_id: {}, name: {}, done: {} ];",
            self.task_id, self.name, self.done
        )
    }
}
