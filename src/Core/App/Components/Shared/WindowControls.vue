<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Plus, X } from "lucide-vue-next";

const appWindow = getCurrentWindow();

const close = () => appWindow.close();
const minimize = () => appWindow.minimize();
const toggleMaximize = () => appWindow.toggleMaximize();
</script>

<template>
	<div class="button-group">
		<button @click="close" class="control-button red">
			<X />
		</button>
		<button @click="minimize" class="control-button yellow">
			<Minus />
		</button>
		<button @click="toggleMaximize" class="control-button green">
			<Plus />
		</button>
	</div>
</template>

<style lang="scss" scoped>
.button-group {
	display: inline-flex;
	position: relative;
	pointer-events: none;
	gap: 8px;
	align-items: center;
	height: 14px;
	padding: 26px 0 0 18px;
	&:hover .control-button svg {
		opacity: 0.9;
	}
}
.control-button {
	position: relative;
	pointer-events: auto;
	cursor: default;
	width: 12px;
	height: 12px;
	border-radius: 50%;
	display: flex;
	justify-content: center;
	align-items: center;
	transition: opacity 0.2s ease;
	padding: 0;
	border: none;
	-webkit-app-region: no-drag;

	svg {
		width: 10px;
		height: 10px;
		opacity: 0;
		transition: opacity 0.2s ease;
		color: var(--black);
	}
}

.red {
	background-color: #ff5f56;
}
.yellow {
	background-color: #ffbd2e;
}
.green {
	background-color: #27c93f;
}
</style>
