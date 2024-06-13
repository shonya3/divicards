import type { NoItemsTab } from '@divicards/shared/poe.types';
import { LitElement, html, css, TemplateResult, nothing } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { BaseElement } from '../base-element';
import { TabBadgeElement } from './tab-badge';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import { ErrorLabel } from './types';

declare global {
	interface HTMLElementTagNameMap {
		'e-stash-tab-errors': StashTabErrorsElement;
	}
}

/**
 * @summary Represents a block of possible stash tab errors during loading.
 * @event upd:errors CustomEvent<Array<{ noItemsTab: NoItemsTab; message: string }>> - Emitted when the errors array changes due to user interaction.
 */
export class StashTabErrorsElement extends BaseElement {
	static override tag = 'e-stash-tab-errors';
	static override get defineList() {
		return [TabBadgeElement];
	}
	@property({ type: Array }) errors: Array<ErrorLabel> = [];

	protected render() {
		if (!this.errors.length) {
			return nothing;
		}
		return html`<ul>
			${this.errors.map(
				({ noItemsTab: tab, message }) =>
					html`<li>
						<wc-tab-badge .tab=${tab}></wc-tab-badge>
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
	}

	static styles = css`
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
			border: 0.5px solid red;
			padding: 0.6rem 0.8rem;
			max-width: fit-content;
			display: grid;
			gap: 1rem;
		}
		li {
			display: flex;
			justify-content: center;
			align-items: center;
			gap: 2rem;
		}
	`;
}
