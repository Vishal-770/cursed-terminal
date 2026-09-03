pub mod commands;

use commands::dispatch_command;
use commands::types::CommandResponse;

#[tauri::command]
fn execute_command(cmd: String) -> CommandResponse {
    std::panic::catch_unwind(|| {
        dispatch_command(cmd)
    })
    .unwrap_or_else(|_| {
        CommandResponse::text(
            "[KERNEL RECOVERY]: Segmentation fault caught by air-gap watchdog.\n\
            System state restored. Did you forget which operating system memory model you violated?"
        )
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![execute_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
