#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hwelu \"{}\"", name)
}

#[tauri::command]
fn generate(min: i64, max: i64) -> i64 {
    min + max
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
