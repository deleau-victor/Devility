use std::process::Command;

/// Redirige un port en utilisant `iptables` (Linux uniquement).
/// ⚠️ Requiert les droits root, donc Tauri devra être lancé avec `sudo`
/// ou une stratégie d’élévation devra être mise en place.
pub fn add_rule(
    listen_address: String,
    listen_port: u16,
    connect_address: String,
    connect_port: u16,
    protocol: String,
) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(&[
            "iptables", "-t", "nat", "-A", "PREROUTING",
            "-p", &protocol,
            "-d", &listen_address,
            "--dport", &listen_port.to_string(),
            "-j", "DNAT",
            "--to-destination", &format!("{}:{}", connect_address, connect_port),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

use crate::commands::port_forwarding::PortRuleDto;
use std::process::Command;

pub fn list_rules() -> Result<Vec<PortRuleDto>, String> {
    let output = Command::new("sudo")
        .args(["iptables", "-t", "nat", "-L", "-n", "-v"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut rules = vec![];

    for line in stdout.lines() {
        if line.contains("REDIRECT") && line.contains("dpt:") {
            // example: REDIRECT tcp -- 0.0.0.0/0 → 0.0.0.0/0 tcp dpt:8080 redir ports 3000
            let listen_port = line.split("dpt:").nth(1)
                .and_then(|p| p.split_whitespace().next())
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(0);

            let connect_port = line.split("redir ports").nth(1)
                .and_then(|p| p.trim().parse::<u16>().ok())
                .unwrap_or(0);

            rules.push(PortRuleDto {
                id: uuid::Uuid::new_v4().to_string(),
                listen_address: "0.0.0.0".to_string(),
                listen_port,
                connect_address: "127.0.0.1".to_string(),
                connect_port,
                protocol: "tcp".to_string(),
            });
        }
    }

    Ok(rules)
}
