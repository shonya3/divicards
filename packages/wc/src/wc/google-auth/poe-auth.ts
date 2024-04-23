import { html, css } from 'lit';
import { property } from 'lit/decorators.js';
import { BaseElement } from '../base-element';

declare global {
	interface HTMLElementTagNameMap {
		'wc-google-auth': GoogleAuthElement;
	}
}

export class GoogleAuthElement extends BaseElement {
	static override tag = 'wc-google-auth';
	static override styles = [styles()];

	@property({ reflect: true }) name: string = '';
	@property({ reflect: true }) picture: string = '';
	@property({ type: Boolean, reflect: true }) loggedIn: boolean = false;
	protected override render() {
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
		this.emit('login');
	}

	#emitLogout() {
		this.emit('logout');
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
