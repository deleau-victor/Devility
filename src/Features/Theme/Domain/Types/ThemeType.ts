import { THEME_TOKENS } from "../Constants/ThemeTokens";

export type Theme = (typeof THEME_TOKENS)[keyof typeof THEME_TOKENS];

export const availableThemes: Theme[] = Object.values(THEME_TOKENS);
