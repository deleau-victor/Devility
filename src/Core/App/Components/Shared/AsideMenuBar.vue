<script lang="ts" async setup>
import { EthernetPort } from "lucide-vue-next";
</script>
<template>
	<aside class="sidebar__container">
		<RouterLink to="/" active-class="active" class="sidebar__link">
			<div class="sidebar__link__icon">
				<EthernetPort />
			</div>
			<p class="sidebar__link__text" translate="no">Redirection de port</p>
		</RouterLink>
		<RouterLink to="/port" active-class="active" class="sidebar__link">
			<div class="sidebar__link__icon">
				<EthernetPort />
			</div>
			<p class="sidebar__link__text" translate="no">Redirection de port</p>
		</RouterLink>
	</aside>
</template>
<style lang="scss" scoped>
.sidebar {
	&__container {
		width: 72px;
		height: 100vh;
		padding: 40px 12px;
		max-width: 72px;
		border-radius: 8px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
	}
	&__link {
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-decoration: none;
		gap: 2px;
		cursor: pointer;
		&:hover > .sidebar__link__icon {
			background-color: rgba(var(--background-invert-accent), 0.2);
		}
		&.active > .sidebar__link__icon {
			background-color: rgb(var(--background-invert-accent));
			& > svg {
				width: 20px;
				height: 20px;
				color: var(--text-invert);
			}
		}
	}
	&__link__icon {
		width: 30px;
		height: 30px;
		padding: 4px;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 6px;
		& > svg {
			width: 20px;
			height: 20px;
			color: var(--text-base);
		}
	}
	&__link__text {
		font-size: 12px;
		font-weight: 500;
		color: var(--text-invert);
		text-align: center;
	}
}
</style>
