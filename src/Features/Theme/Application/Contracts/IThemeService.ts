import { Theme } from "../../Domain/Types/ThemeType";

export interface IThemeService {
	GetTheme(): Theme;
	SetTheme(value: string): void;
}
