import { PortRule } from "@/Features/PortForwarding/Domain/Entities/PortRule";
import { container } from "tsyringe";
import { onMounted, ref } from "vue";
import { PORT_SLICE_TOKENS } from "../DependencyInjection";
import { IPortForwardService } from "./IPortForwardService";

export function usePortForwarding() {
	const portRules = ref<PortRule[]>([]);
	const _service = container.resolve<IPortForwardService>(PORT_SLICE_TOKENS.PortForwardService);

	const LoadPortRules = async () => {
		portRules.value = await _service.List();
	};

	const AddPortRule = async (data: Omit<PortRule, "Id" | "Label">) => {
		const rule = PortRule.Create(data);
		await _service.Add(rule);
		portRules.value.push(rule);
	};

	const RemovePortRule = async (listenPort: number) => {
		await _service.Remove(listenPort);
		portRules.value = portRules.value.filter((rule) => rule.ListenPort !== listenPort);
	};

	onMounted(() => {
		LoadPortRules();
	});

	return {
		portRules,
		LoadPortRules,
		AddPortRule,
		RemovePortRule
	};
}
