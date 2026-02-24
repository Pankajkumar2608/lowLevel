use serde::{Serialize, Deserialize};
use chorno::{DateTime, Utc};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub priority: u8,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}

