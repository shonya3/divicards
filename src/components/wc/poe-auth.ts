import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { property } from 'lit-element/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-poe-auth': PoeAuthElement;
	}
}

const styles = css`
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

export class PoeAuthElement extends BaseElement {
	static htmlTag = 'wc-poe-auth';
	static styles = [styles, this.baseStyles];

	@property({ reflect: true }) name: string = '';
	@property({ type: Boolean, reflect: true }) loggedIn: boolean = false;
	render() {
		const logoutButton = html`<button @click=${this.#emitLogout}>Logout</button>`;
		const loginButton = html`<button @click=${this.#emitLogin}>Login</button>`;

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
