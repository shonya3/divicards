import { EventMapFrom } from '../../event-utils.js';
import { ColorTheme } from './e-theme-toggle.js';

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export type Events = [typeof ChangeThemeEvent];

export class ChangeThemeEvent extends Event {
	static readonly tag = 'theme-toggle__change:theme';

	constructor(public readonly $theme: ColorTheme) {
		super(ChangeThemeEvent.tag, { bubbles: true, composed: true });
	}
}
