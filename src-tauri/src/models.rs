use serde::{Deserialize, Serialize};

// ─── Search ───────────────────────────────────────────────────────────────────

/// A candidate result from yt-dlp's JSON dump.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YtdlpEntry {
    pub id: String,
    pub title: String,
    pub webpage_url: String,
    pub duration: Option<f64>,
    pub view_count: Option<u64>,
    pub uploader: Option<String>,
}

/// The best matching result after scoring, returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub video_id: String,
    pub title: String,
    pub url: String,
    pub duration_secs: f64,
    pub score: i32,
}

// ─── Download events ──────────────────────────────────────────────────────────

/// Status of a single track during download.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrackStatus {
    Pending,
    Searching,
    Downloading,
    Done,
    Error,
}

/// Emitted via Tauri events to the frontend for each track update.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackProgress {
    pub index: usize,
    pub status: TrackStatus,
    pub filename: Option<String>,
    pub error: Option<String>,
}

// ─── History ──────────────────────────────────────────────────────────────────

/// A completed download session persisted in history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistorySession {
    pub id: String,
    pub playlist_name: String,
    pub output_folder: String,
    pub format: String,
    pub total_tracks: usize,
    pub done_tracks: usize,
    pub failed_tracks: usize,
    pub started_at: u64,
    pub completed_at: u64,
    pub elapsed_secs: u64,
}

// ─── Download input ───────────────────────────────────────────────────────────

/// One track to download (sent from frontend to backend).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackInput {
    pub index: usize,
    pub title: String,
    pub artist: String,
    pub album: String,
}

/// Full download session parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub tracks: Vec<TrackInput>,
    pub output_folder: String,
    pub playlist_name: String,
    pub format: String,          // "m4a" | "mp3"
    pub mp3_quality: String,     // "vbr0" | "192" | "128"
    pub search_mode: String,     // "fast" | "deep"
    pub exclude_instrumentals: bool,
    pub duration_min: u32,
    pub duration_max: u32,
    pub generate_m3u: bool,
}
