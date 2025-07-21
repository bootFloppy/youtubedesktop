// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")] {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1"); // nvidia fixes
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODR", "1"); // nvidia fixes
    }
    
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
