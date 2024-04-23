import { html, css, svg } from 'lit';
import { BaseElement } from '../../base-element';
import { property, query } from 'lit/decorators.js';
import { FixedName } from '@divicards/shared/types';
import { BasePopupElement } from '../../base-popup';
import { FixedIconElement } from './fixed-icon';

declare global {
	interface HTMLElementTagNameMap {
		'wc-fixed-names': FixedNamesElement;
	}
}

export class FixedNamesElement extends BaseElement {
	static override get defineList() {
		return [BasePopupElement, FixedIconElement];
	}
	static override tag = 'wc-fixed-names';

	@property({ reflect: true, type: Number }) width = 24;
	@property({ reflect: true, type: Number }) height = 24;
	@property({ type: Array }) fixedNames: FixedName[] = [];

	@query('wc-base-popup') popup!: BasePopupElement;

	#onIconClicked() {
		this.popup.open();
	}

	protected override render() {
		return html`<wc-fixed-icon
				@click=${this.#onIconClicked}
				width=${this.width}
				height=${this.height}
			></wc-fixed-icon>
			<wc-base-popup>
				<div class="fixed-names">
					<h2>Automatically fixed typos</h2>
					<ul class="fixed-names">
						${this.fixedNames.map(({ old, fixed }) => {
							return html`<li class="list-item">
								<span class="input-name">${old}</span>
								${this.arrowIcon()}
								<span class="fixed-name">${fixed}</span>
							</li>`;
						})}
					</ul>
				</div>
			</wc-base-popup> `;
	}

	protected arrowIcon() {
		return html`<svg width="16" height="16" viewBox="0 0 24 24">
			<path fill="currentColor" d="m14 18l-1.4-1.45L16.15 13H4v-2h12.15L12.6 7.45L14 6l6 6Z"></path>
		</svg>`;
	}

	static override styles = [
		css`
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
