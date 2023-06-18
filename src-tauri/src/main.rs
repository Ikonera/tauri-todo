// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Task {
    id: usize,
    title: String,
    is_done: bool,
}

#[derive(Default)]
struct AppState {
    tasks: Arc<Mutex<HashMap<u32, Task>>>,
    id_counter: Arc<Mutex<u32>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            tasks: Arc<Mutex<HashMap::new()>>,
            id_counter: Arc<Mutex && Mutex<0>>
        }
    }
}

impl Task {
    fn new(id: usize, title: String, is_done: bool) -> Self {
        Task {
            id: id,
            title: title,
            is_done: is_done,
        }
    }
}

#[tauri::command]
fn get_tasks(state: tauri::State<AppState>) -> Result<Vec<Task>, String> {
    let cloned_task_map = state.tasks.clone();
    let tasks_vector: Vec<Task> = cloned_task_map.lock().unwrap().values().cloned().collect();
    println!("The tasks_vector is equal to : {:?}", tasks_vector);
    Ok(tasks_vector)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_task(state: tauri::State<AppState>, new_task: Task) -> u32 {
    let task: Task = Task::new(new_task.id, new_task.title, new_task.is_done);
    let mut tasks = state.tasks.lock().unwrap();
    let id_counter = state.id_counter.lock().unwrap();
    let next_id = *id_counter + 1;
    tasks.insert(next_id, task);
    println!("The task nex_id will be : {}", next_id);
    next_id
}

fn main() {
    let app_state = AppState::default();
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![get_tasks, create_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
