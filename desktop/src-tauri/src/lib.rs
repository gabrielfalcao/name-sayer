#![allow(unused)]
#[doc(hidden)] pub mod errors;
#[doc(inline)] pub use errors::{Error, Result};
use tauri::{AppHandle, Emitter, EventTarget, tray::TrayIconBuilder};
use tauri_plugin_dialog::DialogExt;
#[tauri::command]
fn open_file(app: AppHandle) -> Result<Option<String>> {
    let file_path =
        app.dialog().file().add_filter("Text & Markdown", &["txt", "md"]).blocking_pick_file();

    match file_path {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None), // User canceled the dialog
    }
}
#[tauri::command]
fn download(app: AppHandle, url: String) {
    app.emit("download-started", &url).unwrap();
    for progress in [1, 15, 50, 80, 100] {
        app.emit("download-progress", progress).unwrap();
    }
    app.emit("download-finished", &url).unwrap();
}

use iocore::Path;
use name_sayer::BlockedAddress;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn load_blocklist(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let mut names = Vec::<String>::new();
    let lines = path
        .read_lines()
        .unwrap()
        .into_iter()
        .filter(|l| l.trim().len() > 0)
        .map(|s| BlockedAddress::new(s))
        .filter(|a| a.is_ok())
        .map(|a| a.unwrap().to_string())
        .collect::<Vec<String>>();
    lines
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_file, download, load_blocklist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
