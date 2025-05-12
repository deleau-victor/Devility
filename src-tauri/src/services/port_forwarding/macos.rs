use std::process::Command;
use crate::commands::port_forwarding::PortRuleDto;
use uuid::Uuid;

pub fn add_rule(
    _listen_address: String,
    listen_port: u16,
    _connect_address: String,
    connect_port: u16,
    _protocol: String,
) -> Result<(), String> {
    println!(
        "📩 [Rust] Appel de `portforwardd add {}`",
        listen_port
    );

    let output = Command::new("portforwardd")
        .args(["add", &listen_port.to_string(), &connect_port.to_string()])
        .output()
        .map_err(|e| format!("Erreur exécution de portforwardd add : {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "portforwardd add a échoué : {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!(
        "🟢 portforwardd add réussi : {}",
        String::from_utf8_lossy(&output.stdout).trim()
    );
    Ok(())
}

pub fn list_rules() -> Result<Vec<PortRuleDto>, String> {
    let output = Command::new("portforwardd")
        .arg("list")
        .output()
        .map_err(|e| format!("Erreur exécution de portforwardd list : {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "portforwardd list a échoué : {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut rules = vec![];

	for line in stdout.lines() {
		if line.starts_with("rdr pass on lo0") {
			// Exemple : rdr pass on lo0 inet proto tcp from any to any port = 8080 -> 127.0.0.1 port 3000
			let tokens: Vec<&str> = line.split_whitespace().collect();

			// Recherche des positions dynamiques
			if let Some(eq_index) = tokens.iter().position(|t| *t == "=") {
				let listen_port = tokens.get(eq_index + 1).and_then(|s| s.parse().ok()).unwrap_or(0);
				let connect_addr = tokens.get(eq_index + 3).unwrap_or(&"");
				let connect_port = tokens.get(eq_index + 5).and_then(|s| s.parse().ok()).unwrap_or(0);

				rules.push(PortRuleDto {
					id: Uuid::new_v4().to_string(),
					listen_address: "127.0.0.1".to_string(), // toujours redirigé en local
					listen_port,
					connect_address: connect_addr.to_string(),
					connect_port,
					protocol: "tcp".to_string(),
				});
			}
		}
	}

    Ok(rules)
}


pub fn remove_rule(listen_port: u16) -> Result<(), String> {
    println!("📩 [Rust] Appel de `portforwardd remove {}`", listen_port);

    let output = Command::new("portforwardd")
        .args(["remove", &listen_port.to_string()])
        .output()
        .map_err(|e| format!("Erreur exécution de portforwardd remove : {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "portforwardd remove a échoué : {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!(
        "🟢 portforwardd remove réussi : {}",
        String::from_utf8_lossy(&output.stdout).trim()
    );

    Ok(())
}
