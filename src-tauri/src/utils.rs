use std::process::Command;
use tauri::command;

/// Check if mpd is installed.
#[command]
pub(crate) fn check_mpd() -> Result<bool, String> {
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

