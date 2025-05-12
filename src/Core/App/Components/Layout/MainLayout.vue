<template>
	<header data-tauri-drag-region>
		<WindowControls />
	</header>
	<main class="main">
		<AsideMenuBar />
		<section class="app__container">
			<slot />
		</section>
	</main>
</template>
<style scoped>
.app__container {
	margin-top: 14px;
	width: 100%;
	height: 100%;
	min-height: calc(100vh - 51px);
	display: flex;
	flex-direction: column;
	gap: 16px;
	padding: 12px 24px;
	background-color: rgb(var(--background-base));
	border-radius: 12px;
}
</style>
