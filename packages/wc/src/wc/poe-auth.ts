import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-poe-auth': PoeAuthElement;
	}
}

export class PoeAuthElement extends BaseElement {
	static override tag = 'wc-poe-auth';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) name: string = '';
	@property({ type: Boolean, reflect: true }) loggedIn: boolean = false;
	protected override render() {
		const logoutButton = html`<sl-button @click=${this.#emitLogout}>Logout</sl-button>`;
		const loginButton = html`<sl-button @click=${this.#emitLogin}>Login</sl-button>`;

		const template = this.loggedIn
			? html`<div class="logged-in">
					<p>${this.name}</p>
					${logoutButton}
				</div>`
			: html`<div>${loginButton}</div>`;

		return html`<div class="poe-auth">${template}</div>`;
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
		.poe-auth {
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
