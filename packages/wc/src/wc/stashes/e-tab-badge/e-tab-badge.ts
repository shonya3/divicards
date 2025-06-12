import { html, LitElement, CSSResult, TemplateResult } from 'lit';
import { customElement, property, query, state } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';
import { REMOVE_ONLY } from '../e-tab-badge-group/e-tab-badge-group.js';
import type { NoItemsTab } from 'poe-custom-elements/types.js';
import { styles } from './e-tab-badge.styles.js';
import { TabSelectEvent, TabClickEvent } from './events.js';

@customElement('e-tab-badge')
export class TabBadgeElement extends LitElement {
	static styles: Array<CSSResult> = [styles];

	@property({ type: Object }) tab!: NoItemsTab;
	@property({ type: Boolean }) disabled = false;
	@property({ type: Boolean, reflect: true }) selected = false;
	/** Any valid CSS color */
	@property({ reflect: true, attribute: 'color' }) color?: string;
	@property() as: 'button' | 'checkbox' = 'button';

	@state() tabState!: NoItemsTab;

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

		return html`e-tab-badge Error: Unexpected variant of prop 'as': ${this.as}`;
	}

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

	@query('input') checkbox!: HTMLInputElement;
	#set_selected_and_emit() {
		this.selected = this.checkbox.checked;
		this.dispatchEvent(new TabSelectEvent(this.tab, this.selected, { composed: true }));
	}
	#emit_tab_click() {
		this.dispatchEvent(new TabClickEvent(this.tab, { composed: true }));
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'e-tab-badge': TabBadgeElement;
	}
}
