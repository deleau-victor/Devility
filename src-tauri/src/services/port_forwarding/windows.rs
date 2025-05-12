use crate::commands::port_forwarding::PortRuleDto;
use std::process::Command;

pub fn add_rule(
    listen_address: String,
    listen_port: u16,
    connect_address: String,
    connect_port: u16,
    _protocol: String,
) -> Result<(), String> {
    let output = Command::new("netsh")
        .args(&[
            "interface", "portproxy", "add", "v4tov4",
            &format!("listenaddress={}", listen_address),
            &format!("listenport={}", listen_port),
            &format!("connectaddress={}", connect_address),
            &format!("connectport={}", connect_port),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn list_rules() -> Result<Vec<PortRuleDto>, String> {
    let output = Command::new("netsh")
        .args(&["interface", "portproxy", "show", "all"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let mut rules = vec![];

    for line in raw.lines().skip(4) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            rules.push(PortRuleDto {
                id: uuid::Uuid::new_v4().to_string(),
                listen_address: parts[0].to_string(),
                listen_port: parts[1].parse().unwrap_or(0),
                connect_address: parts[2].to_string(),
                connect_port: parts[3].parse().unwrap_or(0),
                protocol: "tcp".to_string(),
            });
        }
    }

    Ok(rules)
}

pub fn remove_rule(listen_address: String, listen_port: u16) -> Result<(), String> {
    let output = Command::new("netsh")
        .args(&[
            "interface", "portproxy", "delete", "v4tov4",
            &format!("listenaddress={}", listen_address),
            &format!("listenport={}", listen_port),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
