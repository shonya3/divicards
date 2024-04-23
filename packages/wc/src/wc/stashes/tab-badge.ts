import { html, css } from 'lit';
import { BaseElement } from '../base-element';
import { property, query } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';
import { REMOVE_ONLY } from './tab-badge-group';

declare global {
	interface HTMLElementTagNameMap {
		'wc-tab-badge': TabBadgeElement;
	}
}

export interface Events {
	'tab-select': { tabId: TabBadgeElement['tabId']; name: TabBadgeElement['name']; selected: boolean };
}

export class TabBadgeElement extends BaseElement {
	static override tag = 'wc-tab-badge';
	/** Color from Poe API. Examples: ff, 80b3ff, #f0f80, cc009a, 7c5436 */
	@property({ reflect: true, attribute: 'hexish-color' }) hexishColor: string = '7c5436';
	/** Any valid CSS color */
	@property({ reflect: true, attribute: 'color' }) color?: string;
	@property({ reflect: true })
	name = 'Heist';
	@property({ reflect: true }) tabId: string = 'e07f5f2946';
	@property({ type: Boolean, reflect: true }) selected = false;
	@property({ type: Number, reflect: true }) index: number = 0;

	@query('input') checkbox!: HTMLInputElement;

	get computedColor(): string {
		return this.color ? this.color : `#${this.hexishColor.padStart(6, '0')}`;
	}

	protected nameLabel() {
		const removeOnly = this.name.includes(REMOVE_ONLY);

		if (removeOnly) {
			const [name] = this.name.split(REMOVE_ONLY);
			return html`<label for=${this.tabId} class="name">${name}<span class="remove-only">R</span></label>`;
		}

		return html`<label for=${this.tabId} class="name">${this.name}</label>`;
	}

	protected override render() {
		const cssProps = styleMap({
			'--badge-color': `${this.computedColor}`,
			'--tab-index': `' ${this.index} '`,
		});

		return html`<div class="tab-badge" style=${cssProps}>
			${this.nameLabel()}
			<input
				@change=${this.#onCheckbox}
				class="checkbox"
				type="checkbox"
				.tabId=${this.tabId}
				.checked=${this.selected}
			/>
		</div>`;
	}

	#onCheckbox() {
		this.selected = this.checkbox.checked;

		this.emit<Events['tab-select']>('tab-select', {
			tabId: this.tabId,
			selected: this.selected,
			name: this.name,
		});
	}

	static styles = css`
		.tab-badge {
			width: 8rem;
			height: 4rem;
			aspect-ratio: 1;
			display: flex;
			justify-content: center;
			align-items: center;

			border-radius: 2rem;
			border: 1px solid #000;
			overflow: clip;

			background-color: var(--badge-color);
			position: relative;

			&:has(.checkbox:checked) {
				transform: scale(1.3);
				z-index: 2;
			}

			&::after {
				display: block;
				position: absolute;
				bottom: 0;
				right: 0;
				background-color: rgba(255, 255, 255, 0.09);
				color: #000;
				content: var(--tab-index);
				width: 2.8rem;
				text-align: center;
				border-top-left-radius: 2rem;
				font-size: 0.8rem;
			}

			.name {
				color: var(--badge-color);
				font-size: 0.9rem;
				color: #000;
				position: relative;

				.remove-only {
					font-size: 60%;
					vertical-align: sub;
				}
			}

			.checkbox {
				position: absolute;
				appearance: none;
				height: 100%;
				width: 100%;
				cursor: pointer;
			}
		}
	`;
}

export type TabSelectEvent = CustomEvent<{ tabId: string; selected: boolean }>;
