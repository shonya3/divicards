import { html, css, LitElement, CSSResult, TemplateResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { emit } from '../utils.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-google-auth': GoogleAuthElement;
	}
}

@customElement('e-google-auth')
export class GoogleAuthElement extends LitElement {
	static override styles: Array<CSSResult> = [styles()];

	@property({ reflect: true }) name: string = '';
	@property({ reflect: true }) picture: string = '';
	@property({ type: Boolean, reflect: true }) loggedIn: boolean = false;
	protected override render(): TemplateResult {
		const logoutButton = html`<button @click=${this.#emitLogout}>Logout</button>`;
		const loginButton = html`<button @click=${this.#emitLogin}>Login Sheets</button>`;

		const template = this.loggedIn
			? html`<div class="logged-in">
					<img src=${this.picture} alt="user avatar" width="32" height="32" />
					<p>${this.name}</p>
					${logoutButton}
			  </div>`
			: html`<div>${loginButton}</div>`;

		return html`<div class="auth">${template}</div>`;
	}

	#emitLogin() {
		emit(this, 'login');
	}

	#emitLogout() {
		emit(this, 'logout');
	}
}

function styles() {
	return css`
		.auth {
			position: relative;
		}
		.logged-in {
			display: flex;
			align-items: center;
			justify-self: center;
			gap: 1rem;
		}

		.logs {
			opacity: 0.6;
			font-size: 80%;
		}
	`;
}
