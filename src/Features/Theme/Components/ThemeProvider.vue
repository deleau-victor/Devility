<script setup lang="ts">
import { useTheme } from "@/Features/Theme/Application/Composables/UseTheme";
import { availableThemes } from "@/Features/Theme/Domain/Types/ThemeType";
import { useHead } from "@unhead/vue";
import { container } from "tsyringe";
import { IThemeService } from "../Application/Contracts/IThemeService";
import { THEME_SLICE_TOKENS } from "../DependencyInjection";

const themeService = container.resolve<IThemeService>(THEME_SLICE_TOKENS.ThemeService);
const { setTheme, resolvedTheme } = useTheme();

onMounted(() => {
	const stored = themeService.GetTheme();
	setTheme(stored);
});

watch(
	resolvedTheme,
	(theme) => {
		useHead({
			title: theme,
			link: [
				{
					rel: "stylesheet",
					href: `/Themes/Variables.${theme}.css`,
					id: "theme-vars"
				}
			]
		});

		document.documentElement.classList.remove(...availableThemes);
		document.documentElement.classList.add(theme);
	},
	{ immediate: true }
);
</script>

<template><slot /></template>
