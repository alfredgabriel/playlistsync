use serde::Serialize;
use std::path::PathBuf;
use tauri::Manager;

/// Result of checking whether yt-dlp and ffmpeg are available on the system.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolStatus {
    pub ytdlp_available: bool,
    pub ytdlp_path: Option<String>,
    pub ffmpeg_available: bool,
    pub ffmpeg_path: Option<String>,
}

pub fn get_tool_path(app: &tauri::AppHandle, tool_name: &str) -> Option<PathBuf> {
    // Try to get from resources (bundled)
    if let Ok(resource_dir) = app.path().resource_dir() {
        let bin_path = resource_dir.join("bin").join(tool_name);
        if bin_path.exists() {
            return Some(bin_path);
        }
    }
    // Fallback to system PATH
    which::which(tool_name).ok()
}

/// Check if yt-dlp and ffmpeg are installed and reachable via PATH or bundled.
/// Returns paths when found so the frontend can display them.
#[tauri::command]
pub fn check_tools(app: tauri::AppHandle) -> ToolStatus {
    let ytdlp = get_tool_path(&app, "yt-dlp.exe");
    let ffmpeg = get_tool_path(&app, "ffmpeg.exe");

    ToolStatus {
        ytdlp_available: ytdlp.is_some(),
        ytdlp_path: ytdlp.as_ref().map(|p| p.display().to_string()),
        ffmpeg_available: ffmpeg.is_some(),
        ffmpeg_path: ffmpeg.as_ref().map(|p| p.display().to_string()),
    }
}

/// Get the version string of yt-dlp by running `yt-dlp --version`.
#[tauri::command]
pub async fn get_ytdlp_version(app: tauri::AppHandle) -> Result<String, String> {
    let ytdlp_path = get_tool_path(&app, "yt-dlp.exe")
        .ok_or_else(|| "yt-dlp not found".to_string())?;

    let output = tokio::process::Command::new(ytdlp_path)
        .arg("--version")
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("yt-dlp not found".to_string())
    }
}
