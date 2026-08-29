use serde::Serialize;
use std::path::PathBuf;

/// Result of checking whether yt-dlp and ffmpeg are available on the system.
#[derive(Debug, Serialize)]
pub struct ToolStatus {
    pub ytdlp_available: bool,
    pub ytdlp_path: Option<String>,
    pub ffmpeg_available: bool,
    pub ffmpeg_path: Option<String>,
}

/// Check if yt-dlp and ffmpeg are installed and reachable via PATH.
/// Returns paths when found so the frontend can display them.
#[tauri::command]
pub fn check_tools() -> ToolStatus {
    let ytdlp = which::which("yt-dlp").ok();
    let ffmpeg = which::which("ffmpeg").ok();

    ToolStatus {
        ytdlp_available: ytdlp.is_some(),
        ytdlp_path: ytdlp.as_ref().map(|p: &PathBuf| p.display().to_string()),
        ffmpeg_available: ffmpeg.is_some(),
        ffmpeg_path: ffmpeg.as_ref().map(|p: &PathBuf| p.display().to_string()),
    }
}

/// Get the version string of yt-dlp by running `yt-dlp --version`.
#[tauri::command]
pub async fn get_ytdlp_version() -> Result<String, String> {
    let output = tokio::process::Command::new("yt-dlp")
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
