import { PortRule } from "../Domain/Entities/PortRule";

export interface IPortForwardService {
	List(): Promise<PortRule[]>;
	Add(rule: PortRule): Promise<void>;
	Remove(listen_port: number): Promise<void>;
}
