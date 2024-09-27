import { html, css, LitElement } from 'lit';
import { customElement, property, query } from 'lit/decorators.js';
import { FixedName } from '@divicards/shared/types';
import { BasePopupElement } from '../../e-base-popup';
import '../../e-base-popup';
import './e-fixed-icon';

declare global {
	interface HTMLElementTagNameMap {
		'e-fixed-names': FixedNamesElement;
	}
}

@customElement('e-fixed-names')
export class FixedNamesElement extends LitElement {
	@property({ reflect: true, type: Number }) width = 24;
	@property({ reflect: true, type: Number }) height = 24;
	@property({ type: Array }) fixedNames: FixedName[] = [];

	@query('e-base-popup') popup!: BasePopupElement;

	#onIconClicked() {
		this.popup.showModal();
	}

	protected override render() {
		return html`<e-fixed-icon
				@click=${this.#onIconClicked}
				width=${this.width}
				height=${this.height}
			></e-fixed-icon>
			<e-base-popup>
				<div class="content">
					<h2 class="heading">Automatically fixed typos</h2>
					<ul class="fixed-names-list">
						${this.fixedNames.map(({ old, fixed }) => {
							return html`<li class="list-item">
								<span class="input-name">${old}</span>
								${this.arrowIcon()}
								<span class="fixed-name">${fixed}</span>
							</li>`;
						})}
					</ul>
				</div>
			</e-base-popup> `;
	}

	protected arrowIcon() {
		return html`<svg width="16" height="16" viewBox="0 0 24 24">
			<path fill="currentColor" d="m14 18l-1.4-1.45L16.15 13H4v-2h12.15L12.6 7.45L14 6l6 6Z"></path>
		</svg>`;
	}

	static override styles = [
		css`
			.heading {
				font-weight: 500;
			}

			.content {
				padding: 2rem;
			}

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
		`,
	];
}
