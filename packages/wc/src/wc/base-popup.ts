import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { query } from 'lit/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-base-popup': BasePopupElement;
	}
}

export class BasePopupElement extends BaseElement {
	static override tag = 'wc-base-popup';
	static override styles = [this.baseStyles, styles()];

	@query('dialog') dialog!: HTMLDialogElement;

	protected override render() {
		return html`<dialog>
			<slot></slot>
			<button @click=${() => this.dialog.close()} class="btn-close">X</button>
		</dialog> `;
	}

	open() {
		if (!this.dialog) {
			this.updateComplete.then(() => {
				return this.dialog.showModal();
			});
		} else {
			return this.dialog.showModal();
		}
	}

	show() {
		return this.dialog.showModal();
	}

	hide() {
		return this.dialog.close();
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
			animation: content-fade-in 800ms forwards ease-out;
		}

		dialog::backdrop {
			background-color: darkorange;
			background-image: linear-gradient(130deg, #ff7a18, #af002d 41.07%, #319197 76.05%);
			filter: blur(0px);
			animation: backdrop-fade-in 800ms forwards ease-out;
			backdrop-filter: blur(20px);
		}

		dialog::slotted(*) {
			margin-inline: auto;
			padding: 0.8rem;
			padding-top: 1.6rem;
		}

		@media (min-width: 800px) {
			::slotted(*) {
				padding-inline: 2rem;
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
