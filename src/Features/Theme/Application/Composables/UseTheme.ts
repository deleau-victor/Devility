import { THEME_TOKENS } from "../../Domain/Constants/ThemeTokens";
import { availableThemes, Theme } from "../../Domain/Types/ThemeType";

const theme = ref<Theme>(THEME_TOKENS.SYSTEM);

const prefersDark = window.matchMedia("(prefers-color-scheme: dark)");

const resolvedTheme = computed(() =>
	theme.value === THEME_TOKENS.SYSTEM
		? prefersDark.matches
			? THEME_TOKENS.DARK
			: THEME_TOKENS.LIGHT
		: theme.value
);

export function useTheme() {
	return {
		theme,
		resolvedTheme,
		availableThemes,
		setTheme: (val: Theme) => (theme.value = val)
	};
}
