<script lang="ts" async setup>
import { usePortForwarding } from "@/Features/PortForwarding/Application/usePortForwarding";
import { PortRule } from "@/Features/PortForwarding/Domain/Entities/PortRule";

const { portRules, LoadPortRules, RemovePortRule } = usePortForwarding();

const remove = async (rule: PortRule) => {
	await RemovePortRule(rule.ListenPort);
	await LoadPortRules();
};
</script>
<template>
	<div class="forward-container">
		<div class="forward-header">
			<h2>Règles de redirection</h2>
			<AddPortForm />
		</div>

		<div class="table-wrapper" v-if="portRules.length > 0">
			<table class="port-table">
				<thead>
					<tr>
						<th>Port d'écoute</th>
						<th>Port cible</th>
						<th>Protocole</th>
						<th>Action</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="rule in portRules" :key="rule.Id">
						<td>{{ rule.ListenPort }}</td>
						<td>{{ rule.ConnectPort }}</td>
						<td class="capitalize">{{ rule.Protocol.toLocaleUpperCase() }}</td>
						<td>
							<button class="action-button" @click="remove(rule)">Supprimer</button>
						</td>
					</tr>
				</tbody>
			</table>
		</div>

		<div class="empty-state" v-else>
			<p>Aucune règle active</p>
		</div>
	</div>
</template>
<style lang="scss" scoped>
.forward-container {
	padding: 24px;
	width: 100%;
	max-width: 900px;
	display: flex;
	flex-direction: column;
	gap: 16px;
}

.forward-header {
	display: flex;
	justify-content: space-between;
	align-items: center;

	h2 {
		font-size: 1.4rem;
		font-weight: 600;
		color: var(--text-base);
	}

	.add-button {
		background: rgb(var(--background-invert-accent));
		color: var(--text-invert);
		padding: 8px 16px;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-weight: 500;

		&:hover {
			background: rgba(var(--background-invert-accent), 0.8);
		}
	}
}

.table-wrapper {
	border-radius: 8px;
	overflow: hidden;
	box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.05);
}

.port-table {
	width: 100%;
	border-collapse: collapse;

	thead {
		background: rgba(var(--background-invert-accent), 0.1);
		position: sticky;
		top: 0;

		th {
			text-align: left;
			padding: 12px 16px;
			font-size: 0.85rem;
			text-transform: uppercase;
			color: var(--text-accent);
			letter-spacing: 0.05em;
		}
	}

	tbody {
		tr {
			background: rgba(var(--background-invert-accent), 0.04);
			&:nth-child(even) {
				background: rgba(var(--background-invert-accent), 0.06);
			}

			td {
				padding: 12px 16px;
				color: var(--text-base);
			}
		}
	}
}

.action-button {
	background: none;
	border: none;
	color: rgb(var(--background-invert-accent));
	cursor: pointer;
	font-weight: 500;

	&:hover {
		text-decoration: underline;
	}
}

.empty-state {
	text-align: center;
	padding: 40px 0;
	color: var(--text-subtle);
	font-size: 1rem;
}
</style>
