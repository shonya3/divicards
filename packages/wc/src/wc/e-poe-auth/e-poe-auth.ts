import { html, css, LitElement, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import { Events, LoginClickEvent, LogoutClickEvent } from './events.js';
import { DefineComponent } from 'vue';
import { VueEventHandlers } from '../../event-utils.js';

export type PoeAuthProps = {
	auth: AuthState;

	/**
	 * The button's size.
	 */
	size?: ButtonSize;
};

export type AuthState = { loggedIn: true; username: string } | { loggedIn: false };
export type ButtonSize = 'small' | 'medium' | 'large';

@customElement('e-poe-auth')
export class PoeAuthElement extends LitElement {
	static override styles: Array<CSSResult> = [styles()];

	@property({ type: Object }) auth: AuthState = { loggedIn: false };

	/** The button's size. */
	@property()
	size: ButtonSize = 'small';

	protected override render(): TemplateResult {
		return html`<div class="poe-auth">
			${this.auth.loggedIn
				? html`<div class="logged-in">
						<p>${this.name_without_hash}</p>
						<sl-button .size=${this.size} @click=${this.#emitLogout}>Logout</sl-button>
				  </div>`
				: html`<div>
						<sl-button .size=${this.size} @click=${this.#emitLogin}>Login</sl-button>
				  </div>`}
		</div>`;
	}

	/** Get the name without the hash part. */
	get name_without_hash(): string | null {
		if (!this.auth.loggedIn) return null;

		const name = this.auth.username;

		const hash_index = name.indexOf('#');
		if (hash_index === -1) return name;

		return name.slice(0, hash_index);
	}

	#emitLogin() {
		this.dispatchEvent(new LoginClickEvent());
	}

	#emitLogout() {
		console.log('logout click');
		this.dispatchEvent(new LogoutClickEvent());
	}
}

function styles() {
	return css`
		.logged-in {
			display: flex;
			align-items: center;
			justify-self: center;
			gap: 1rem;
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'e-poe-auth': PoeAuthElement;
	}
}

declare module 'vue' {
	interface GlobalComponents {
		'e-poe-auth': DefineComponent<PoeAuthProps & VueEventHandlers<Events>>;
	}
}
