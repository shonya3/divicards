import { html, css, LitElement } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import { emit } from '../utils';

@customElement('e-poe-auth')
export class PoeAuthElement extends LitElement {
	static override styles = [styles()];
	@property({ reflect: true }) name: string = '';
	@property({ type: Boolean, reflect: true }) loggedIn: boolean = false;

	protected override render() {
		return html`<div class="poe-auth">
			${this.loggedIn
				? html`<div class="logged-in">
						<p>${this.name}</p>
						<sl-button @click=${this.#emitLogout}>Logout</sl-button>
					</div>`
				: html`<div>
						<sl-button @click=${this.#emitLogin}>Login</sl-button>
					</div>`}
		</div>`;
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
