import { html, css, PropertyValueMap } from 'lit';
import { BaseElement } from '../base-element';
import { property, query, state } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';
import { REMOVE_ONLY } from './tab-badge-group';
import { NoItemsTab } from '@divicards/shared/poe.types';

declare global {
	interface HTMLElementTagNameMap {
		'wc-tab-badge': TabBadgeElement;
	}
}

export interface Events {
	'tab-select': { tabId: string; name: string; selected: boolean };
}

export class TabBadgeElement extends BaseElement {
	static override tag = 'wc-tab-badge';
	@property({ type: Object }) tab!: NoItemsTab;
	@property({ type: Boolean, reflect: true }) selected = false;
	// /** Any valid CSS color */
	@property({ reflect: true, attribute: 'color' }) color?: string;

	@state() tabState!: NoItemsTab;

	get computedColor(): string {
		if (this.color) {
			return this.color;
		}
		if (this.tab.metadata?.colour) {
			return `#${this.tab.metadata?.colour?.padStart(6, '0')}`;
		}
		return '#fff';
	}

	protected nameLabel() {
		const removeOnly = this.tab.name.includes(REMOVE_ONLY);

		if (removeOnly) {
			const [name] = this.tab.name.split(REMOVE_ONLY);
			return html`<label for=${this.tab.id} class="name">${name}<span class="remove-only">R</span></label>`;
		}

		return html`<label for=${this.tab.id} class="name">${this.tab.name}</label>`;
	}

	protected override render() {
		const cssProps = styleMap({
			'--badge-color': `${this.computedColor}`,
			'--tab-index': `' ${this.tab.index} '`,
		});

		return html`<div class="tab-badge" style=${cssProps}>
			${this.nameLabel()}
			<input
				@change=${this.#onCheckbox}
				class="checkbox"
				type="checkbox"
				.tabId=${this.tab.id}
				.checked=${this.selected}
			/>
		</div>`;
	}

	@query('input') checkbox!: HTMLInputElement;
	#onCheckbox() {
		this.selected = this.checkbox.checked;

		this.emit<Events['tab-select']>('tab-select', {
			tabId: this.tab.id,
			selected: this.selected,
			name: this.tab.name,
		});
	}

	static styles = css`
		.tab-badge {
			width: 5.5rem;
			height: 2.2rem;
			aspect-ratio: 1;
			display: flex;
			justify-content: center;
			align-items: center;
			border-radius: 0.4rem;
			border: 1px solid #000;
			overflow: clip;
			background-color: var(--badge-color);
			position: relative;
			&:has(.checkbox:checked) {
				transform: scale(1.3);
				z-index: 2;
			}

			.name {
				color: var(--badge-color);
				font-size: 0.85rem;
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

			&::after {
				display: block;
				position: absolute;
				bottom: 0;
				right: 0;
				background-color: rgba(255, 255, 255, 0.06);
				color: #000;
				content: var(--tab-index);
				text-align: center;
				border-top-left-radius: 2rem;
				font-size: 0.6rem;
				min-width: 1rem;
			}
		}
	`;
}

export type TabSelectEvent = CustomEvent<{ tabId: string; selected: boolean }>;
