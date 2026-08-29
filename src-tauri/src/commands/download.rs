use crate::models::{HistorySession, TrackInput, TrackProgress, TrackStatus};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadOptions {
    pub tracks: Vec<TrackInput>,
    pub output_folder: String,
    pub playlist_name: String,
    pub format: String,
    pub mp3_quality: String,
    pub search_mode: String,
    pub exclude_instrumentals: bool,
    pub duration_min: u32,
    pub duration_max: u32,
    pub generate_m3u: bool,
}

#[derive(Default)]
pub struct DownloadState {
    pub cancel_token: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
}

#[tauri::command]
pub async fn start_download_session(
    app: AppHandle,
    state: State<'_, DownloadState>,
    options: DownloadOptions,
) -> Result<(), String> {
    let (tx, mut rx) = tokio::sync::oneshot::channel();
    {
        let mut cancel_token = state.cancel_token.lock().await;
        *cancel_token = Some(tx);
    }

    let output_dir = PathBuf::from(&options.output_folder);
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output dir: {}", e))?;
    }

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut done_count = 0;
    let mut fail_count = 0;
    let total = options.tracks.len();

    let mut m3u_lines = Vec::new();
    if options.generate_m3u {
        m3u_lines.push("#EXTM3U".to_string());
    }

    for track in &options.tracks {
        // check cancel
        if rx.try_recv().is_ok() {
            println!("Download cancelled");
            break;
        }

        // emit searching
        emit_progress(&app, track.index, TrackStatus::Searching, None, None);
        
        let search_res = crate::commands::search::search_track(
            app.clone(),
            track.title.clone(),
            track.artist.clone(),
            track.album.clone(),
            options.search_mode == "deep",
            options.duration_min,
            options.duration_max,
            options.exclude_instrumentals,
        ).await;

        match search_res {
            Ok(best) => {
                emit_progress(&app, track.index, TrackStatus::Downloading, None, None);
                
                let file_ext = if options.format == "m4a" { "m4a" } else { "mp3" };
                let sanitized_title = sanitize_filename(&track.title);
                let sanitized_artist = sanitize_filename(&track.artist);
                let filename = format!("{:02}_{} - {}.{}", track.index + 1, sanitized_artist, sanitized_title, file_ext);
                let filepath = output_dir.join(&filename);

                let url = format!("https://www.youtube.com/watch?v={}", best.video_id);
                
                let ytdlp_path = crate::commands::tools::get_tool_path(&app, "yt-dlp.exe")
                    .ok_or_else(|| "yt-dlp not found.".to_string())?;
                let mut cmd = tokio::process::Command::new(ytdlp_path);
                
                cmd.args(["--no-warnings", "--quiet"]);

                if options.format == "m4a" {
                    cmd.args(["-f", "bestaudio[ext=m4a]"]);
                } else {
                    let ffmpeg_path = crate::commands::tools::get_tool_path(&app, "ffmpeg.exe")
                        .ok_or_else(|| "ffmpeg not found.".to_string())?;
                        
                    cmd.args([
                        "-x",
                        "--audio-format", "mp3",
                        "--audio-quality", &options.mp3_quality,
                        "--ffmpeg-location", &ffmpeg_path.to_string_lossy(),
                    ]);
                }
                
                cmd.arg("-o").arg(filepath.to_string_lossy().to_string());
                cmd.arg(&url);

                #[cfg(target_os = "windows")]
                let create_no_window = 0x08000000;
                #[cfg(target_os = "windows")]
                cmd.creation_flags(create_no_window);

                let dl_res = cmd.output().await;

                if let Ok(out) = dl_res {
                    if out.status.success() {
                        emit_progress(&app, track.index, TrackStatus::Done, Some(filename.clone()), None);
                        if options.generate_m3u {
                            m3u_lines.push(filename);
                        }
                        done_count += 1;
                    } else {
                        emit_progress(&app, track.index, TrackStatus::Error, None, Some("Download failed".to_string()));
                        fail_count += 1;
                    }
                } else {
                    emit_progress(&app, track.index, TrackStatus::Error, None, Some("yt-dlp execution failed".to_string()));
                    fail_count += 1;
                }
            }
            Err(e) => {
                emit_progress(&app, track.index, TrackStatus::Error, None, Some(e));
                fail_count += 1;
            }
        }
    }

    if options.generate_m3u && !m3u_lines.is_empty() {
        let m3u_path = output_dir.join(format!("{}.m3u", options.playlist_name));
        let _ = fs::write(m3u_path, m3u_lines.join("\n"));
    }

    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let session = HistorySession {
        id: Uuid::new_v4().to_string(),
        playlist_name: options.playlist_name,
        output_folder: options.output_folder,
        format: options.format,
        total_tracks: total,
        done_tracks: done_count,
        failed_tracks: fail_count,
        started_at: start_time,
        completed_at: end_time,
        elapsed_secs: end_time.saturating_sub(start_time),
    };

    crate::commands::history::add_to_history(session);

    Ok(())
}

#[tauri::command]
pub async fn cancel_download(state: State<'_, DownloadState>) -> Result<(), String> {
    let mut cancel_token = state.cancel_token.lock().await;
    if let Some(tx) = cancel_token.take() {
        let _ = tx.send(());
    }
    Ok(())
}

fn emit_progress(app: &AppHandle, index: usize, status: TrackStatus, filename: Option<String>, error: Option<String>) {
    let prog = TrackProgress {
        index,
        status,
        filename,
        error,
    };
    let _ = app.emit("download:progress", prog);
}

fn sanitize_filename(name: &str) -> String {
    name.replace(&['\\', '/', ':', '*', '?', '"', '<', '>', '|'][..], "")
}
