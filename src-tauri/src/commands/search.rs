use crate::models::{SearchResult, YtdlpEntry};
use crate::commands::tools::get_tool_path;

/// Penalty keywords that indicate an undesired result (instrumental, karaoke, live, etc.).
/// Mirrors the scoring logic from spotify2media.py.
const PENALTY_KEYWORDS: &[&str] = &[
    "instrumental",
    "karaoke",
    "live",
    "cover",
    "remix",
    "tribute",
    "acoustic",
    "extended",
    "nightcore",
    "slowed",
    "reverb",
    "sped up",
];

/// Score a candidate result.
/// - Penalises if title contains unwanted keywords and the original track title doesn't
/// - Penalises heavy duration deviation (>30s off from expected duration)
fn score_entry(
    entry: &YtdlpEntry,
    title: &str,
    artist: &str,
    duration_min: u32,
    duration_max: u32,
    exclude_instrumentals: bool,
) -> i32 {
    let mut score: i32 = 100;
    let entry_lower = entry.title.to_lowercase();
    let title_lower = title.to_lowercase();

    // Penalise unwanted variants
    for kw in PENALTY_KEYWORDS {
        let in_result = entry_lower.contains(kw);
        let in_original = title_lower.contains(kw);
        if in_result && !in_original {
            if exclude_instrumentals && (*kw == "instrumental" || *kw == "karaoke") {
                score -= 50; // heavy penalty
            } else {
                score -= 20;
            }
        }
    }

    // Reward if artist name appears in title/uploader
    let artist_lower = artist.to_lowercase();
    if entry_lower.contains(&artist_lower)
        || entry
            .uploader
            .as_ref()
            .map(|u| u.to_lowercase().contains(&artist_lower))
            .unwrap_or(false)
    {
        score += 15;
    }

    // Penalise if duration is out of range
    if let Some(dur) = entry.duration {
        if dur < duration_min as f64 || dur > duration_max as f64 {
            score -= 80; // effectively filters it out
        }
    } else {
        score -= 10; // no duration info = slight penalty
    }

    score
}

/// Search YouTube Music for the best match for a given track.
/// Uses yt-dlp's `ytsearch` with either 1 result (fast) or 5 results (deep).
#[tauri::command]
pub async fn search_track(
    app: tauri::AppHandle,
    title: String,
    artist: String,
    _album: String,
    deep: bool,
    duration_min: u32,
    duration_max: u32,
    exclude_instrumentals: bool,
) -> Result<SearchResult, String> {
    let result_count = if deep { 5 } else { 1 };
    let query = format!("{} {}", title, artist);
    let search_query = format!("ytsearch{}:{}", result_count, query);

    #[cfg(target_os = "windows")]
    let create_no_window = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        CREATE_NO_WINDOW
    };

    let ytdlp_path = get_tool_path(&app, "yt-dlp.exe")
        .ok_or_else(|| "yt-dlp not found. Please install it or use the bundled version.".to_string())?;

    let mut cmd = tokio::process::Command::new(ytdlp_path);
    cmd.args([
        "--dump-json",
        "--no-playlist",
        "--skip-download",
        "--no-warnings",
        "--quiet",
        &search_query,
    ]);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(create_no_window);

    let output = cmd.output().await.map_err(|e| {
        format!("yt-dlp failed to run: {}", e)
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp search error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries: Vec<YtdlpEntry> = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<YtdlpEntry>(line).ok())
        .collect();

    if entries.is_empty() {
        return Err(format!("No results found for: {} - {}", title, artist));
    }

    // Score all entries and pick the best one
    let mut scored: Vec<(i32, YtdlpEntry)> = entries
        .drain(..)
        .map(|e| {
            let s = score_entry(
                &e,
                &title,
                &artist,
                duration_min,
                duration_max,
                exclude_instrumentals,
            );
            (s, e)
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    let (score, best) = scored.remove(0);

    if score < 0 {
        return Err(format!(
            "No acceptable result found for: {} - {} (best score: {})",
            title, artist, score
        ));
    }

    Ok(SearchResult {
        video_id: best.id,
        title: best.title,
        url: best.webpage_url,
        duration_secs: best.duration.unwrap_or(0.0),
        score,
    })
}
