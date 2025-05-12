use tauri::command;
use crate::services::port_forwarding;
use serde::Serialize;

#[derive(Serialize)]
pub struct PortRuleDto {
    pub id: String,
    pub listen_address: String,
    pub listen_port: u16,
    pub connect_address: String,
    pub connect_port: u16,
    pub protocol: String,
}


#[command]
pub fn add_port_rule(
    listen_address: String,
    listen_port: u16,
    connect_address: String,
    connect_port: u16,
    protocol: String
) -> Result<(), String> {
	println!("📩 [Rust] Appel de add_port_rule()");
    port_forwarding::add_rule(
        listen_address,
        listen_port,
        connect_address,
        connect_port,
        protocol,
    )
}

#[tauri::command]
pub fn list_port_rules() -> Result<Vec<PortRuleDto>, String> {
	println!("📩 [Rust] Appel de list_port_rule()");
    port_forwarding::list_rules()
}

#[tauri::command]
pub fn remove_port_rule(listen_address: Option<String>, listen_port: u16) -> Result<(), String> {
	println!("📩 [Rust] Appel de remove_port_rule()");
	port_forwarding::remove_rule(listen_address.unwrap_or("127.0.0.1".into()), listen_port)
}

