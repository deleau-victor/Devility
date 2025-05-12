export class PortRule {
	public readonly Id: string;
	public readonly ListenAddress: string;
	public readonly ListenPort: number;
	public readonly ConnectAddress: string;
	public readonly ConnectPort: number;
	public readonly Protocol: "tcp" | "udp";

	constructor(
		id: string,
		listenAddress: string,
		listenPort: number,
		connectAddress: string,
		connectPort: number,
		protocol: "tcp" | "udp"
	) {
		this.Id = id;
		this.ListenAddress = listenAddress;
		this.ListenPort = listenPort;
		this.ConnectAddress = connectAddress;
		this.ConnectPort = connectPort;
		this.Protocol = protocol;
	}

	public get Label(): string {
		return `${this.ListenAddress}:${this.ListenPort} → ${this.ConnectAddress}:${this.ConnectPort} (${this.Protocol})`;
	}

	public static Create(data: Omit<PortRule, "Id" | "Label">): PortRule {
		return new PortRule(
			crypto.randomUUID(),
			data.ListenAddress,
			data.ListenPort,
			data.ConnectAddress,
			data.ConnectPort,
			data.Protocol
		);
	}
}
