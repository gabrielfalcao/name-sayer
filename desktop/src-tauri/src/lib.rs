#![allow(unused)]
#[doc(hidden)] pub mod errors;
#[doc(inline)] pub use errors::{Error, Result};
use serde::Serialize;
use tauri::{
    AppHandle,
    Emitter,
    EventTarget,
    ipc::Channel,
    tray::TrayIconBuilder,
};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn open_file(app: AppHandle) {
    app.dialog().file().pick_file(move |file_path| match file_path {
        Some(path) => {
            app.emit("file-selected", &path.to_string()).unwrap();
            println!("{path}");
        }
        None => {
            eprintln!("no path");
        }
    });
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
        .invoke_handler(tauri::generate_handler![greet, open_file, load_blocklist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
