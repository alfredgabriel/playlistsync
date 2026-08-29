mod commands;
mod models;

use commands::{config, tools, search, download, history};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(download::DownloadState::default())
        .invoke_handler(tauri::generate_handler![
            // tools
            tools::check_tools,
            tools::get_ytdlp_version,
            // config
            config::load_app_config,
            config::save_app_config,
            // search
            search::search_track,
            // download
            download::start_download_session,
            download::cancel_download,
            // history
            history::load_history,
            history::save_history,
            history::clear_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
