import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { query } from 'lit-element/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-base-popup': BasePopupElement;
	}
}

// const styles = css`
// 	@keyframes blur-in {
// 		from {
// 			backdrop-filter: blur(0px) brightness(100%);
// 		}
// 		to {
// 			backdrop-filter: blur(3px) brightness(40%);
// 		}
// 	}

// 	.popup {
// 		/* backdrop-filter: brightness(40%) blur(3px); */
// 		animation: blur-in 200ms forwards;

// 		position: absolute;
// 		top: 0;
// 		left: 0;
// 		width: 100%;
// 		height: 100%;
// 		display: flex;
// 		justify-content: center;
// 		align-items: center;
// 		z-index: 3;
// 	}
// 	.backdrop {
// 		opacity: 70%;
// 		width: 100%;
// 		height: 100%;
// 		top: 0;
// 		left: 0;
// 		position: absolute;

// 		z-index: 3;
// 	}

// 	.popup_content {
// 		color: var(--color);
// 		background-color: var(--bg-color);
// 		width: min(95%, 1200px);
// 		padding: 5rem;

// 		overflow-y: scroll;
// 		height: 90vh;

// 		/* background-color: #fff; */
// 		border-radius: 4px;
// 		z-index: 10;
// 		top: 0;

// 		z-index: 4;
// 	}
// `;

const styles = css`
	:host {
		display: block;
	}
	.popup_content {
		margin-inline: auto;
		display: block;
		color: var(--color);
		background-color: var(--bg-color);
		width: min(95%, 1200px);
		padding: 5rem;

		overflow-y: scroll;
		height: 90vh;

		/* background-color: #fff; */
		border-radius: 4px;
		z-index: 10;
		top: 0;

		z-index: 4;
	}
`;

export class BasePopupElement extends BaseElement {
	static htmlTag = 'wc-base-popup';
	static styles = [this.baseStyles, styles];

	@query('dialog') dialog!: HTMLDialogElement;

	render() {
		return html`<dialog class="popup">
			<div class="popup_content">
				<slot></slot>
			</div>
		</dialog>`;
	}

	show() {
		return this.dialog.showModal();
	}

	hide() {
		return this.dialog.close();
	}

	connectedCallback(): void {
		super.connectedCallback();
		window.addEventListener('keydown', this.#onEscapePressed.bind(this));
	}

	disconnectedCallback(): void {
		super.disconnectedCallback();
		window.addEventListener('keydown', this.#onEscapePressed.bind(this));
	}

	#onEscapePressed(e: KeyboardEvent) {
		if (e.code === 'Escape') {
			console.log(this);
			this.hide();
		}
	}
}
