import { html, css, PropertyValueMap } from 'lit';
import { BaseElement } from './base-element';
import { property, query } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-base-popup': BasePopupElement;
	}
}

export class BasePopupElement extends BaseElement {
	static override tag = 'wc-base-popup';
	static override styles = [styles()];
	/** Instead of dialog's non-modal open, runs showModal() if true https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog#open */
	@property({ type: Boolean }) open = false;

	protected async willUpdate(map: PropertyValueMap<this>): Promise<void> {
		if (map.has('open')) {
			const dialog = await this.dialog();
			if (this.open) {
				dialog.showModal();
			} else {
				dialog.close();
			}
		}
	}

	async dialog(): Promise<HTMLDialogElement> {
		const queryDialog = () => this.shadowRoot!.querySelector('dialog');
		const dialog = queryDialog();

		if (dialog) {
			return dialog;
		} else {
			await this.updateComplete;
			return queryDialog()!;
		}
	}

	protected override render() {
		return html`<dialog>
			<div class="slot-parent">
				<slot></slot>
			</div>
			<sl-icon-button name="x-lg" @click=${() => (this.open = false)} class="btn-close">X</sl-icon-button>
		</dialog> `;
	}

	async showModal(): Promise<void> {
		this.open = true;
		await this.updateComplete;
	}

	async close(): Promise<void> {
		this.open = false;
		await this.updateComplete;
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

		dialog::slotted(*) {
			margin-inline: auto;
			padding: 0.8rem;
			padding-top: 1.6rem;
		}

		@media (min-width: 800px) {
			.slot-parent {
				padding: 2rem;
			}
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
