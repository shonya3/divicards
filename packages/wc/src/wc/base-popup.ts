import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { property, query } from 'lit/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-base-popup': BasePopupElement;
	}
}

const styles = css`
	:host {
		display: block;
	}

	dialog {
		animation: fade-in 2000ms forwards cubic-bezier(0.175, 0.885, 0.32, 1.275);
	}

	dialog:modal {
		margin: auto;
		border: none;
		width: min(95%, 1200px);
		border-radius: 10px;
	}

	dialog::backdrop {
		background-color: darkorange;
		background-image: linear-gradient(130deg, #ff7a18, #af002d 41.07%, #319197 76.05%);
		filter: blur(0px);
		animation: blur-in 2000ms forwards cubic-bezier(0.175, 0.885, 0.32, 1.275);
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

	@keyframes blur-in {
		from {
			filter: blur(0px) brightness(0);
		}
		to {
			filter: blur(10px) brightness(40%);
		}
	}

	@keyframes fade-in {
		from {
			filter: brightness(0);
		}
		to {
			filter: brightness(100%);
		}
	}

	.btn-close {
		position: fixed;
		top: 0;
		right: 0;
		z-index: 3;
	}
`;

export class BasePopupElement extends BaseElement {
	static htmlTag = 'wc-base-popup';
	static styles = [this.baseStyles, styles];

	@query('dialog') dialog!: HTMLDialogElement;

	render() {
		return html`<dialog>
			<slot></slot>
			<button @click=${() => this.dialog.close()} class="btn-close">X</button>
		</dialog> `;
	}

	open() {
		return this.dialog.showModal();
	}

	show() {
		return this.dialog.showModal();
	}

	hide() {
		return this.dialog.close();
	}

	connectedCallback(): void {
		super.connectedCallback();
	}

	disconnectedCallback(): void {
		super.disconnectedCallback();
	}

	#onEscapePressed(e: KeyboardEvent) {
		if (e.code === 'Escape') {
			console.log(this);
			this.hide();
		}
	}
}
