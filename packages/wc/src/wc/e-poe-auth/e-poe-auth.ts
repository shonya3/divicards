import { html, css, LitElement, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import { Events, LoginClickEvent, LogoutClickEvent } from './events.js';
import { DefineComponent } from 'vue';
import { VueEventHandlers } from '../../event-utils.js';

export type PoeAuthProps = {
	auth: AuthState;
};

export type AuthState = { loggedIn: true; username: string } | { loggedIn: false };

@customElement('e-poe-auth')
export class PoeAuthElement extends LitElement {
	static override styles: Array<CSSResult> = [styles()];

	@property({ type: Object }) auth: AuthState = { loggedIn: false };

	protected override render(): TemplateResult {
		return html`<div class="poe-auth">
			${this.auth.loggedIn
				? html`<div class="logged-in">
						<p>${this.auth.username}</p>
						<sl-button @click=${this.#emitLogout}>Logout</sl-button>
				  </div>`
				: html`<div>
						<sl-button @click=${this.#emitLogin}>Login</sl-button>
				  </div>`}
		</div>`;
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
