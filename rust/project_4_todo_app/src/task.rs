use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use timeago::Formatter;
use std::time::SystemTime;


let formatter = Formatter::new();
let now = SystemTime::now();

let created: SystemTime = task.created_at.into();
let human_time = formatter.convert(created, now);

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub priority: u8,
    pub created_at: human_time,
    pub due_date: Option<DateTime<Utc>>,
}

