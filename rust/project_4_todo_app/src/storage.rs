use std::fs;
use std::path::PathBuf;
use create::task::Task;

pub fn get_data_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("rust");
    path.push("project_4_todo_app");
    std::fs::create_dir_all(&path).unwrap();
    path.push("todo.txt");
    path
}

pub fn load_tasks() -> Vec<Task> {
    let path = get_data_path();
    if !path.exists() {
        return Vec![];
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_task(tasks: &Vec<Task>) {
    let path = get_data_path();
    let content = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(&path, content).unwrap();
}