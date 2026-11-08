use serde::{Deserialize, Serialize};
use std::format;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreate {
    pub name: String,
    pub done: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdate {
    pub id: Uuid,
    pub name: String,
    pub done: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn to_string(self) -> String {
        format!(
            "Task[ id: {}, name: {}, done: {} ];",
            self.id, self.name, self.done
        )
    }
}
