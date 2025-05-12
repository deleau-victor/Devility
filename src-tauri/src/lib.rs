// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod commands;
pub mod services;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
			commands::port_forwarding::add_port_rule,
    		commands::port_forwarding::list_port_rules,
			commands::port_forwarding::remove_port_rule
			])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
