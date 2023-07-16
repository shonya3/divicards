import { html, css } from 'lit';
import { BaseElement } from '../base-element';
import { property, query } from 'lit-element/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-tab-badge': TabBadgeElement;
	}
}

const styles = css`
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
			transform: scale(1.4);
			z-index: 2;
		}

		&:after {
			display: block;
			position: absolute;
			bottom: 0;
			right: 0;
			background-color: rgba(255, 255, 255, 0.3);
			color: #000;
			content: var(--tab-index);
			width: 2.8rem;
			text-align: center;
			border-top-left-radius: 2rem;
			font-size: 0.8rem;
		}

		.name {
			color: var(--badge-color);
			mix-blend-mode: difference;
			font-size: 0.9rem;
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

export class TabBadgeElement extends BaseElement {
	static htmlTag = 'wc-tab-badge';
	static styles = [this.baseStyles, styles];
	@property({ reflect: true }) colour: string = '#FF0000';
	@property({ reflect: true }) name = 'Tab';
	@property({ reflect: true }) tabId: string = '1';
	@property({ type: Boolean, reflect: true }) selected = false;
	@property({ type: Number, reflect: true }) index: number = 0;

	@query('input') checkbox!: HTMLInputElement;

	get color(): string {
		return `#${this.colour.padStart(6, '0')}`;
	}

	get checked(): boolean {
		return this.checkbox.checked;
	}

	render() {
		const cssProps = styleMap({
			'--badge-color': `${this.color}`,
			'--tab-index': `' ${this.index} '`,
		});

		return html`<div class="tab-badge" style=${cssProps}>
			<label for=${this.tabId} class="name">${this.name}</label>
			<input
				@change=${this.#emitTabSelect}
				class="checkbox"
				type="checkbox"
				.tabId=${this.tabId}
				.checked=${this.selected}
			/>
		</div>`;
	}

	#emitTabSelect() {
		const detail = {
			tabId: this.tabId,
			selected: this.checked,
		};
		this.emit('tab-select', detail);
	}
}

export type TabSelectEvent = CustomEvent<{ tabId: string; selected: boolean }>;
