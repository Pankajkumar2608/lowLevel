use crate::task::Task;
use crate::storage::{load_tasks, save_tasks};
use uuid::Uuid;
use chrono::Utc;

pub struct TodoApp {
    pub tasks: Vec<Task>,
}

impl TodoApp {
    pub fn new() -> Self {
        Self {
            tasks: load_tasks(),
        }
    }

    pub fn add(&mut self, title: String, priority: u8) {
        let task = Task {
            id: Uuid::new_v4(),
            title,
            completed: false,
            priority,
            created_at: Utc::now(),
            due_date: None,
        };

        self.tasks.push(task);
        save_tasks(&self.tasks);
    }

    pub fn complete(&mut self, id: Uuid) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            save_tasks(&self.tasks);
        }
    }

    pub fn delete(&mut self, id: Uuid) {
        self.tasks.retain(|t| t.id != id);
        save_tasks(&self.tasks);
    }
}