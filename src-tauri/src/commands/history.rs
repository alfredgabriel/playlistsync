use crate::models::HistorySession;
use std::fs;
use std::path::PathBuf;

fn history_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("playlistsync").join("history.json"))
}

#[tauri::command]
pub fn load_history() -> Vec<HistorySession> {
    if let Some(path) = history_path() {
        if path.exists() {
            if let Ok(raw) = fs::read_to_string(&path) {
                if let Ok(history) = serde_json::from_str::<Vec<HistorySession>>(&raw) {
                    return history;
                }
            }
        }
    }
    vec![]
}

pub fn add_to_history(session: HistorySession) {
    let mut history = load_history();
    history.push(session);
    
    // Sort by started_at descending (newest first)
    history.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    
    // Keep only last 50
    history.truncate(50);
    
    if let Some(path) = history_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&history) {
            let _ = fs::write(&path, json);
        }
    }
}

#[tauri::command]
pub fn save_history(history: Vec<HistorySession>) -> Result<(), String> {
    let path = history_path().ok_or("Could not determine history directory")?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(&history).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn clear_history() -> Result<(), String> {
    save_history(vec![])
}
