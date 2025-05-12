<script lang="ts" async setup>
import { X } from "lucide-vue-next";
import { usePortForwarding } from "../Application/usePortForwarding";

const _listenPort = ref<number | null>(null);
const listenPort = computed<number>({
	get() {
		const val = Number(_listenPort.value);
		return isNaN(val) ? 0 : val;
	},
	set(v: number) {
		_listenPort.value = v;
	}
});

const _connectPort = ref<number | null>(null);
const connectPort = computed<number>({
	get() {
		const val = Number(_connectPort.value);
		return isNaN(val) ? 0 : val;
	},
	set(v: number) {
		_connectPort.value = v;
	}
});
const protocol = ref<"tcp" | "udp">("tcp");

const errorMessage = ref("");

const { AddPortRule, LoadPortRules } = usePortForwarding();

const submit = async () => {
	errorMessage.value = "";

	if (!listenPort.value || !connectPort.value) {
		errorMessage.value = "Tous les champs sont obligatoires.";
		return;
	}

	if (listenPort.value <= 0 || connectPort.value <= 0) {
		errorMessage.value = "Les ports doivent être supérieurs à 0.";
		return;
	}

	try {
		await AddPortRule({
			ListenAddress: "127.0.0.1",
			ListenPort: listenPort.value,
			ConnectAddress: "127.0.0.1",
			ConnectPort: connectPort.value,
			Protocol: protocol.value
		});

		await LoadPortRules();
	} catch (err) {
		errorMessage.value = "Erreur lors de l'ajout de la règle.";
		console.error(err);
	}
};
</script>
<template>
	<DialogRoot>
		<DialogTrigger class="dialog__trigger">Ajouter</DialogTrigger>
		<DialogPortal>
			<DialogOverlay class="dialog__overlay" />
			<DialogContent class="dialog__content">
				<DialogTitle class="dialog__title">Ajouter une redirection</DialogTitle>
				<DialogDescription class="dialog__description">
					Spécifiez les ports et le protocole pour créer une redirection locale.
				</DialogDescription>

				<div class="dialog__fields">
					<div class="dialog__field">
						<label class="dialog__label" for="listenPort">Port d'écoute</label>
						<input
							v-model="_listenPort"
							id="listenPort"
							class="dialog__input"
							placeholder="8080"
						/>
					</div>

					<div class="dialog__field">
						<label class="dialog__label" for="connectPort">Port de destination</label>
						<input
							v-model="_connectPort"
							id="connectPort"
							class="dialog__input"
							placeholder="3000"
						/>
					</div>

					<div class="dialog__field">
						<label class="dialog__label" for="protocol">Protocole</label>
						<select v-model="protocol" id="protocol" class="dialog__input">
							<option value="tcp">TCP</option>
							<option value="udp">UDP</option>
						</select>
					</div>
				</div>

				<div class="dialog__footer">
					<DialogClose as-child>
						<button class="dialog__button" @click.prevent="submit">Ajouter</button>
					</DialogClose>
				</div>

				<DialogClose class="dialog__close" aria-label="Fermer">
					<X />
				</DialogClose>
			</DialogContent>
		</DialogPortal>
	</DialogRoot>
</template>
<style lang="scss" scoped>
.dialog {
	&__trigger {
		background: rgb(var(--background-primary));
		color: var(--text-invert);
		font-weight: 500;
		padding: 8px 16px;
		border-radius: 8px;
		cursor: pointer;
		transition: background 0.2s ease;

		&:hover {
			background: rgb(var(--background-primary-accent));
		}
	}

	&__overlay {
		position: fixed;
		inset: 0;
		background: rgba(var(--black-value), 0.5);
		animation: overlayShow 150ms ease;
	}

	&__content {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: rgb(var(--background-accent));
		border-radius: 12px;
		padding: 32px 24px;
		width: 100%;
		max-width: 480px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25);
		animation: contentShow 150ms ease;
	}

	&__title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-base);
		margin-bottom: 4px;
	}

	&__description {
		color: var(--text-muted);
		font-size: 0.9rem;
		margin-bottom: 24px;
	}

	&__fields {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	&__field {
		display: flex;
		flex-direction: column;
	}

	&__label {
		font-size: 0.85rem;
		color: var(--text-primary);
		margin-bottom: 6px;
	}

	&__input {
		all: unset;
		background: rgba(var(--text-base), 0.05);
		border-radius: 6px;
		padding: 10px 12px;
		color: var(--text-base);
		font-size: 0.95rem;
		box-shadow: 0 0 0 1px rgba(var(--text-primary), 0.4);
		transition: box-shadow 0.2s ease;

		&:focus {
			box-shadow: 0 0 0 2px rgba(var(--text-primary-accent), 0.6);
		}
	}

	&__footer {
		display: flex;
		justify-content: flex-end;
		margin-top: 24px;
	}

	&__button {
		background: rgb(var(--background-primary));
		color: var(--text-invert);
		border: none;
		padding: 10px 20px;
		border-radius: 8px;
		cursor: pointer;
		font-weight: 500;

		&:hover {
			background: rgb(var(--background-primary-accent));
		}
	}

	&__close {
		position: absolute;
		top: 12px;
		right: 12px;
		background: none;
		border: none;
		color: var(--text-base);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border-radius: 50%;

		&:hover {
			background: rgba(var(--background-primary-accent), 0.1);
		}
	}
}

@keyframes overlayShow {
	from {
		opacity: 0;
	}
	to {
		opacity: 1;
	}
}

@keyframes contentShow {
	from {
		opacity: 0;
		transform: translate(-50%, -48%) scale(0.96);
	}
	to {
		opacity: 1;
		transform: translate(-50%, -50%) scale(1);
	}
}
</style>
