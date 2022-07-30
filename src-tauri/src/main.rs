#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use rand::{Rng, SeedableRng, rngs::StdRng};

struct Generator(Mutex<StdRng>);

#[tauri::command]
fn generate(generator: tauri::State<Generator>, min: i64, max: i64) -> i64 {
    generator.0.lock().unwrap().gen_range(min..(max+1))
}

fn main() {
    tauri::Builder::default()
        .manage(Generator(StdRng::from_entropy().into()))
        .invoke_handler(tauri::generate_handler![generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
