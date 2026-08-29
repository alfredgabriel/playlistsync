use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Full application configuration persisted in AppData.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Audio format: "m4a" or "mp3"
    pub format: String,
    /// MP3 quality: "vbr0", "192", "128"
    pub mp3_quality: String,
    /// Search mode: "fast" (1 result) or "deep" (5 results + scoring)
    pub search_mode: String,
    /// Exclude instrumental/karaoke versions from results
    pub exclude_instrumentals: bool,
    /// Minimum track duration in seconds
    pub duration_min: u32,
    /// Maximum track duration in seconds
    pub duration_max: u32,
    /// Generate .m3u playlist file after download
    pub generate_m3u: bool,
    /// Extra search variant keywords (e.g. ["official audio", "lyrics"])
    pub variants: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            format: "m4a".to_string(),
            mp3_quality: "vbr0".to_string(),
            search_mode: "deep".to_string(),
            exclude_instrumentals: false,
            duration_min: 30,
            duration_max: 600,
            generate_m3u: true,
            variants: vec![],
        }
    }
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("playlistsync").join("config.json"))
}

/// Load configuration from AppData. Falls back to defaults on error.
#[tauri::command]
pub fn load_app_config() -> AppConfig {
    if let Some(path) = config_path() {
        if path.exists() {
            if let Ok(raw) = fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str::<AppConfig>(&raw) {
                    return cfg;
                }
            }
        }
    }
    AppConfig::default()
}

/// Persist configuration to AppData.
#[tauri::command]
pub fn save_app_config(config: AppConfig) -> Result<(), String> {
    let path = config_path().ok_or("Could not determine config directory")?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}
