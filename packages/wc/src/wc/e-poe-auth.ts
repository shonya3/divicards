import { html, css, LitElement } from 'lit';
import { BaseElement } from './base-element';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import { emit } from '../utils';

declare global {
	interface HTMLElementTagNameMap {
		'e-poe-auth': PoeAuthElement;
	}
}

@customElement('e-poe-auth')
export class PoeAuthElement extends LitElement {
	static override styles = [styles()];

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
		emit(this, 'login');
	}

	#emitLogout() {
		emit(this, 'logout');
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
