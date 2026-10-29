// Empêche la console Windows en mode release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    // Détection si lancé par Firefox ou Chrome via Native Messaging
    if args.iter().any(|a| a.contains("chrome-extension://") || a.contains("firefox")) {
        // Boucle d'écoute stdio JSON
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("Erreur au lancement d'OpenDM");
}
