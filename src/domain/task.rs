
pub struct Task {
    pub id: u32,
    pub name: String,
    pub done: bool
}

impl Task {
    pub fn to_string(self) -> String {
        format!("Task[ id: {}, name: {}, done: {} ];", self.id, self.name, self.done)
    }
}