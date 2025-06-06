import { html, css, LitElement, CSSResult, TemplateResult } from 'lit';
import { customElement, property, query, state } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';
import { REMOVE_ONLY } from './e-tab-badge-group.js';
import type { NoItemsTab } from 'poe-custom-elements/types.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-tab-badge': TabBadgeElement;
	}
}

export type Events = {
	[TabSelectEvent.tag]: TabSelectEvent;
	[TabClickEvent.tag]: TabClickEvent;
};

@customElement('e-tab-badge')
export class TabBadgeElement extends LitElement {
	@property({ type: Object }) tab!: NoItemsTab;
	@property({ type: Boolean }) disabled = false;
	@property({ type: Boolean, reflect: true }) selected = false;
	// /** Any valid CSS color */
	@property({ reflect: true, attribute: 'color' }) color?: string;
	@property() as: 'button' | 'checkbox' = 'button';

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

	protected nameLabel(): TemplateResult {
		const removeOnly = this.tab.name.includes(REMOVE_ONLY);

		if (removeOnly) {
			const [name] = this.tab.name.split(REMOVE_ONLY);
			return html`<label for=${this.tab.id} class="name">${name}<span class="remove-only">R</span></label>`;
		}

		return html`<label for=${this.tab.id} class="name">${this.tab.name}</label>`;
	}

	protected override render(): TemplateResult {
		const cssProps = styleMap({
			'--badge-color': `${this.computedColor}`,
			'--tab-index': `' ${this.tab.index} '`,
		});

		if (this.as === 'checkbox') {
			return html`<div class="tab-badge-as-checkbox" style=${cssProps}>
				${this.nameLabel()}
				<input
					@change=${this.#set_selected_and_emit}
					class="checkbox"
					type="checkbox"
					.checked=${this.selected}
				/>
			</div>`;
		}

		if (this.as === 'button') {
			return html`<button
				.disabled=${this.disabled}
				@click=${this.#emit_tab_click}
				style=${cssProps}
				class="tab-badge-as-button"
			>
				${this.nameLabel()}
			</button>`;
		}

		throw new Error('never');
	}

	@query('input') checkbox!: HTMLInputElement;
	#set_selected_and_emit() {
		this.selected = this.checkbox.checked;
		this.dispatchEvent(new TabSelectEvent(this.tab, this.selected, { composed: true }));
	}
	#emit_tab_click() {
		this.dispatchEvent(new TabClickEvent(this.tab, { composed: true }));
	}

	static styles: CSSResult = css`
		.tab-badge-as-button {
			background-color: var(--badge-color);
			width: 5.5rem;
			height: 2.2rem;
			border-radius: 0.4rem;
			border: 1px solid #000;
			cursor: pointer;
			overflow: hidden;
			position: relative;
			&:hover {
			}
			&:disabled {
				filter: grayscale(0.6);
			}
			.name {
				pointer-events: none;
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

		.tab-badge-as-checkbox {
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

declare global {
	interface HTMLElementEventMap {
		'stashes__tab-select': TabSelectEvent;
		'stashes__tab-click': TabClickEvent;
	}
}

export class TabSelectEvent extends Event {
	static readonly tag = 'stashes__tab-select';
	tab: NoItemsTab;
	selected: boolean;
	constructor(tab: NoItemsTab, selected: boolean, options?: EventInit) {
		super(TabSelectEvent.tag, options);
		this.tab = tab;
		this.selected = selected;
	}
}

export class TabClickEvent extends Event {
	static readonly tag = 'stashes__tab-click';
	tab: NoItemsTab;
	constructor(tab: NoItemsTab, options?: EventInit) {
		super(TabClickEvent.tag, options);
		this.tab = tab;
	}
}
