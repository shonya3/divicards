import { EventMapFrom } from '../../event-utils.js';

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export type Events = [typeof LoginClickEvent, typeof LogoutClickEvent];

export class LoginClickEvent extends Event {
	static readonly tag = 'poe-auth__login';

	constructor() {
		super(LoginClickEvent.tag);
	}
}

export class LogoutClickEvent extends Event {
	static readonly tag = 'poe-auth__logout';

	constructor() {
		super(LogoutClickEvent.tag);
	}
}
