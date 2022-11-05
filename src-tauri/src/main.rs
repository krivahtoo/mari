#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{command, generate_handler, Builder};

mod utils;

#[command]
fn play() -> String {
    "Playing".to_owned()
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![utils::check_mpd, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
