import { html, css, LitElement, CSSResult, TemplateResult } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import './e-tab-badge/e-tab-badge';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import { ErrorLabel } from './types.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-stash-tab-errors': StashTabErrorsElement;
	}
}

/**
 * @summary Represents a block of possible stash tab errors during loading.
 * @event upd:errors CustomEvent<Array<{ noItemsTab: NoItemsTab; message: string }>> - Emitted when the errors array changes due to user interaction.
 * @event upd:hoveredErrorTabId CustomEvent<string | null> - Emitted on Error block mouseenter or mouseleave
 */
@customElement('e-stash-tab-errors')
export class StashTabErrorsElement extends LitElement {
	@property({ type: Array }) errors: Array<ErrorLabel> = [];
	@state() hoveredErrorTabId: ErrorLabel['noItemsTab']['id'] | null = null;

	protected render(): TemplateResult | null {
		if (!this.errors.length) {
			return null;
		}
		return html`<ul>
			<h3>Errors</h3>
			${this.errors.map(
				({ noItemsTab: tab, message }) =>
					html`<li @mouseenter=${() => this.#handleMouseEnter(tab.id)} @mouseleave=${this.#handleMouseLeave}>
						<e-tab-badge .tab=${tab}></e-tab-badge>
						<p>${message}</p>
						<sl-icon-button name="x-lg" @click=${() => this.#handleCloseClick(tab.id)} class="btn-close"
							>X</sl-icon-button
						>
					</li>`
			)}
		</ul>`;
	}

	#handleCloseClick(id: string) {
		const detail = this.errors.filter(error => error.noItemsTab.id !== id);
		this.dispatchEvent(new CustomEvent('upd:errors', { detail }));
		this.dispatchEvent(new CustomEvent('upd:hoveredErrorTabId', { detail: null }));
	}
	#handleMouseEnter(tabId: string) {
		this.hoveredErrorTabId = tabId;
		this.dispatchEvent(new CustomEvent('upd:hoveredErrorTabId', { detail: tabId }));
	}
	#handleMouseLeave() {
		this.hoveredErrorTabId = null;
		this.dispatchEvent(new CustomEvent('upd:hoveredErrorTabId', { detail: null }));
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			display: block;
			height: fit-content;
		}
		ul {
			list-style: none;
			padding: 0.6rem 0.8rem;
			max-width: fit-content;
			display: grid;
			gap: 1rem;
			padding-block: 2rem;
			padding-left: 2rem;
		}
		li {
			display: flex;
			justify-content: center;
			align-items: center;
			gap: 2rem;
		}
		h3 {
			font-weight: 500;
		}
	`;
}
