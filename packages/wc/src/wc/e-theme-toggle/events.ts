import { ColorTheme } from './e-theme-toggle.js';

export type Events = [typeof ChangeThemeEvent];

declare global {
	interface HTMLElementEventMap {
		'theme-toggle__change:theme': ChangeThemeEvent;
	}
}
export class ChangeThemeEvent extends Event {
	static readonly tag = 'theme-toggle__change:theme';

	constructor(public readonly $theme: ColorTheme) {
		super(ChangeThemeEvent.tag, { bubbles: true, composed: true });
	}
}
