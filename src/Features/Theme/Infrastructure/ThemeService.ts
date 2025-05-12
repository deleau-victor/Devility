import { injectable } from "tsyringe";
import { IThemeService } from "../Application/Contracts/IThemeService";
import { THEME_KEY } from "../Domain/Constants/ThemeKey";
import { THEME_TOKENS } from "../Domain/Constants/ThemeTokens";
import { Theme } from "../Domain/Types/ThemeType";

@injectable()
export class ThemeService implements IThemeService {
	private readonly _key = THEME_KEY;

	public constructor() {
		this.init();
	}

	private init(): void {
		const storedTheme = localStorage.getItem(this._key);
		if (storedTheme) return;
		this.SetTheme(this.GetTheme());
	}

	public GetTheme(): Theme {
		const storedTheme = localStorage.getItem(this._key);
		if (storedTheme) {
			return storedTheme as Theme;
		}
		const prefersDark = window.matchMedia("(prefers-color-scheme: dark)");
		if (prefersDark.matches) {
			return THEME_TOKENS.DARK;
		}
		return THEME_TOKENS.LIGHT;
	}

	public SetTheme(value: string): void {
		localStorage.setItem(this._key, value);
	}
}
