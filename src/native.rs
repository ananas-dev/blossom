use std::process::Command;
use std::str;

#[cfg(target_os = "linux")]
pub fn save_file(filename: &str) -> Option<String> {
    let output = Command::new("zenity")
        .args([
            "--file-selection",
            "--save",
            &format!("--filename=\"{}\"", filename),
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout).ok()
}
