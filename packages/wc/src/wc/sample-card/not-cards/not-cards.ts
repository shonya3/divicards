import { html, css } from 'lit';
import { BaseElement } from '../../base-element';
import { property, query } from 'lit/decorators.js';
import { BasePopupElement } from '../../base-popup';

declare global {
	interface HTMLElementTagNameMap {
		'wc-not-cards': NotCardsElement;
	}
}

export class NotCardsElement extends BaseElement {
	static override get defineList() {
		return [BasePopupElement];
	}
	static override tag = 'wc-not-cards';
	static override styles = [styles()];

	@property({ type: Array }) notCards: string[] = [];
	@query('wc-base-popup') popup!: BasePopupElement;

	protected override render() {
		return html`
			${this.warningIcon()}
			<wc-base-popup>
				<div class="fixed-names">
					<h2>Probably not cards</h2>
					<ul class="list">
						${this.notCards.map(notCard => html`<li>${notCard}</li>`)}
					</ul>
				</div>
			</wc-base-popup>
		`;
	}

	#onIconClicked() {
		this.popup.showModal();
	}

	protected warningIcon(size: number = 20) {
		return html`<svg
			@click=${this.#onIconClicked}
			xmlns="http://www.w3.org/2000/svg"
			width=${size}
			:height=${size}
			viewBox="0 0 36 36"
		>
			<path
				fill="#FFCC4D"
				d="M2.653 35C.811 35-.001 33.662.847 32.027L16.456 1.972c.849-1.635 2.238-1.635 3.087 0l15.609 30.056c.85 1.634.037 2.972-1.805 2.972H2.653z"
			/>
			<path
				fill="#231F20"
				d="M15.583 28.953a2.421 2.421 0 0 1 2.419-2.418a2.421 2.421 0 0 1 2.418 2.418a2.422 2.422 0 0 1-2.418 2.419a2.422 2.422 0 0 1-2.419-2.419zm.186-18.293c0-1.302.961-2.108 2.232-2.108c1.241 0 2.233.837 2.233 2.108v11.938c0 1.271-.992 2.108-2.233 2.108c-1.271 0-2.232-.807-2.232-2.108V10.66z"
			/>
		</svg>`;
	}
}

function styles() {
	return css`
		.fixed-names-list {
			margin-top: 2rem;
		}

		.list-item {
			display: flex;
			align-items: center;
			gap: 20px;
		}

		.input-name {
			overflow-x: hidden;
			width: 200px;
			opacity: 60%;
		}

		.list {
			margin-top: 1rem;
		}

		svg {
			cursor: pointer;
		}
	`;
}
