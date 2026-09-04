pub mod commands;

use std::sync::Mutex;
use commands::dispatch_command;
use commands::enclave::EnclaveState;
use commands::types::CommandResponse;

pub type AppState = Mutex<EnclaveState>;

#[tauri::command]
fn execute_command(state: tauri::State<AppState>, cmd: String) -> CommandResponse {
    let mut enclave = state.lock().unwrap();
    dispatch_command(&mut enclave, cmd)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(EnclaveState::default()))
        .invoke_handler(tauri::generate_handler![execute_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
