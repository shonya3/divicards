export type Events = [typeof LoginClickEvent, typeof LogoutClickEvent];

declare global {
	interface HTMLElementEventMap {
		'poe-auth__login': LoginClickEvent;
	}
}
export class LoginClickEvent extends Event {
	static readonly tag = 'poe-auth__login';

	constructor() {
		super(LoginClickEvent.tag);
	}
}

declare global {
	interface HTMLElementEventMap {
		'poe-auth__logout': LogoutClickEvent;
	}
}
export class LogoutClickEvent extends Event {
	static readonly tag = 'poe-auth__logout';

	constructor() {
		super(LogoutClickEvent.tag);
	}
}
