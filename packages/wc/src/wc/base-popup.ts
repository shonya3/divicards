import { html, css, PropertyValueMap, LitElement } from 'lit';
import { BaseElement } from './base-element';
import { customElement, property, query } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-base-popup': BasePopupElement;
	}
}

@customElement('e-base-popup')
export class BasePopupElement extends LitElement {
	static override styles = [styles()];
	/** Instead of dialog's non-modal open, runs showModal() if true https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog#open */
	@property({ type: Boolean, reflect: true }) open = false;

	constructor() {
		super();
		this.addEventListener('click', e => {
			if (e.target === this) {
				this.close();
			}
		});
	}

	protected async willUpdate(map: PropertyValueMap<this>): Promise<void> {
		if (map.has('open')) {
			const dialog = await this.#dialog();
			if (this.open) {
				dialog.showModal();
			} else {
				dialog.close();
			}
		}
	}

	protected override render() {
		return html`<dialog>
			<div class="slot-parent">
				<slot></slot>
			</div>
			<sl-button @click=${this.close} class="btn-close">Close</sl-button>
		</dialog> `;
	}

	async #dialog(): Promise<HTMLDialogElement> {
		const queryDialog = () => this.shadowRoot!.querySelector('dialog');
		const dialog = queryDialog();

		if (dialog) {
			return dialog;
		} else {
			await this.updateComplete;
			return queryDialog()!;
		}
	}

	async showModal(): Promise<void> {
		this.open = true;
		await this.updateComplete;
	}

	async close(): Promise<void> {
		this.open = false;
		await this.updateComplete;
	}

	onEscape = (e: KeyboardEvent) => {
		if (e.code === 'Escape') {
			this.open = false;
		}
	};

	connectedCallback(): void {
		super.connectedCallback();
		window.addEventListener('keydown', this.onEscape);
	}

	disconnectedCallback(): void {
		super.disconnectedCallback();
		window.removeEventListener('keydown', this.onEscape);
	}
}

function styles() {
	return css`
		:host {
			display: block;
			width: min(95%, 1220px);
		}

		dialog {
			margin: 0;
			padding: 0;
		}

		dialog:modal {
			margin: auto;
			border: none;
			border-radius: 10px;
			animation: content-fade-in 300ms forwards ease-out;
		}

		dialog::backdrop {
			filter: blur(0px);
			animation: backdrop-fade-in 300ms forwards ease-out;
			max-width: 100%;
			backdrop-filter: blur(100px);
		}

		@keyframes backdrop-fade-in {
			from {
				filter: brightness(0%);
				opacity: 10%;
			}
			to {
				filter: brightness(60%);
				opacity: 90%;
			}
		}

		@keyframes content-fade-in {
			from {
				opacity: 0%;
			}
			to {
				opacity: 100%;
			}
		}

		.btn-close {
			position: fixed;
			top: 1rem;
			right: 1rem;
			z-index: 3;
		}
	`;
}
