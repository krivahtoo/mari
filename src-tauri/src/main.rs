#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;

use tauri::{command, generate_handler, Builder};

#[command]
fn play() -> String {
    "Playing".to_owned()
}

#[command]
fn check_mpd() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    let res = Command::new("cmd")
        .args(["/C", "mpd -V"])
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute process");
    #[cfg(not(target_os = "windows"))]
    let res = Command::new("sh")
        .arg("-c")
        .arg("mpv -V")
        .output()
        .expect("failed to execute process");
    Ok(res.status.success())
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![check_mpd, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
