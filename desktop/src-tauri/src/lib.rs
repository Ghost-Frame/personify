mod commands;

use commands::personas::{list_personas, active_persona, activate_persona, install_persona};
use commands::growth::get_growth;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_personas,
            active_persona,
            activate_persona,
            install_persona,
            get_growth,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
