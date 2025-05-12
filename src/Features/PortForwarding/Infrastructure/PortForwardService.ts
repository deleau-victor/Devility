import { PortRule } from "@/Features/PortForwarding/Domain/Entities/PortRule";
import { invoke } from "@tauri-apps/api/core";
import { injectable } from "tsyringe";
import { IPortForwardService } from "../Application/IPortForwardService";

@injectable()
export class PortForwardService implements IPortForwardService {
	public async List(): Promise<PortRule[]> {
		const dtos = await invoke<PortRuleDto[]>("list_port_rules");
		console.log(
			dtos.map(
				(dto) =>
					new PortRule(
						dto.id,
						dto.listen_address,
						dto.listen_port,
						dto.connect_address,
						dto.connect_port,
						dto.protocol as "tcp" | "udp"
					)
			)
		);

		return dtos.map(
			(dto) =>
				new PortRule(
					dto.id,
					dto.listen_address,
					dto.listen_port,
					dto.connect_address,
					dto.connect_port,
					dto.protocol as "tcp" | "udp"
				)
		);
	}

	public async Add(rule: PortRule): Promise<void> {
		try {
			await invoke("add_port_rule", {
				listenAddress: rule.ListenAddress,
				listenPort: rule.ListenPort,
				connectAddress: rule.ConnectAddress,
				connectPort: rule.ConnectPort,
				protocol: rule.Protocol
			});
		} catch (error) {
			console.error("Erreur lors de l'ajout de la règle :", error);
			throw error;
		}
	}

	public async Remove(listen_port: number): Promise<void> {
		try {
			await invoke("remove_port_rule", { listenPort: listen_port });
		} catch (error) {
			console.error("Erreur lors de la suppression de la règle :", error);
			throw error;
		}
	}
}
