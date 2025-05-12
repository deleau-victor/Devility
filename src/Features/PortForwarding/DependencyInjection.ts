import { DependencyContainer } from "tsyringe";
import { IPortForwardService } from "./Application/IPortForwardService";
import { PortForwardService } from "./Infrastructure/PortForwardService";

export const PORT_SLICE_TOKENS = {
	PortForwardService: Symbol("PortForwardService")
};

export const RegisterPortSlice = (container: DependencyContainer) => {
	container.register<IPortForwardService>(
		PORT_SLICE_TOKENS.PortForwardService,
		PortForwardService
	);
};
