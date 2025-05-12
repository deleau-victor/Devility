import { DependencyContainer } from "tsyringe";
import { ThemeService } from "./Infrastructure/ThemeService";

export const THEME_SLICE_TOKENS = {
	ThemeService: Symbol("ThemeService")
};

export const RegisterThemeSlice = (container: DependencyContainer) => {
	container.registerSingleton(THEME_SLICE_TOKENS.ThemeService, ThemeService);
};
