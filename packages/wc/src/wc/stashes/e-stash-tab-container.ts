// import 'poe-custom-elements/src/elements/poe-stash-tab';
import '../poe-custom-elements.js';
import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import { TabBadgeElement } from './tab-badge';

import { LitElement, html, css, TemplateResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { NoItemsTab, TabWithItems } from '@divicards/shared/poe.types';

declare global {
	interface HTMLElementTagNameMap {
		'e-stash-tab-container': StashTabContainerElement;
	}
}

/**
 * @summary Container for poe stash tab with header with actions.
 * @event extract-cards - Emitted on "Extract cards sample" button click.
 * @event close - Emitted on "X" button click.
 */
@customElement('e-stash-tab-container')
export class StashTabContainerElement extends LitElement {
	/** PoE API tab data https://www.pathofexile.com/developer/docs/reference#stashes-get */
	@property({ type: Object }) tab: TabWithItems | null = null;
	@property() status: 'pending' | 'complete' = 'pending';
	@property({ type: Object }) badge: NoItemsTab | null = null;

	constructor() {
		super();
		TabBadgeElement.define();
	}

	protected render(): TemplateResult {
		return html`<header class="header">
				${this.badge ? html`<wc-tab-badge as="button" .tab=${this.badge}></wc-tab-badge>` : null}
				${this.status === 'complete'
					? html`<sl-button @click=${this.#emitExtractCards}>Extract cards sample</sl-button>`
					: null}
				<sl-icon-button name="x-lg" @click=${this.#emitClose} class="btn-close">X</sl-icon-button>
			</header>
			<div class="tab-box">
				${this.tab && this.status === 'complete'
					? html`<poe-stash-tab .tab=${this.tab}></poe-stash-tab>`
					: html`<sl-spinner></sl-spinner>`}
			</div>`;
	}

	#emitExtractCards() {
		this.dispatchEvent(new Event('extract-cards'));
	}
	#emitClose() {
		this.dispatchEvent(new Event('close'));
	}

	static styles = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			display: inline-block;
			width: fit-content;
			height: fit-content;
			border: 1px solid var(--sl-color-gray-200);
		}

		.tab-box {
			--size: 569px;
			width: var(--size);
			height: var(--size);
			display: flex;
			align-items: center;
			justify-content: center;
		}

		.header {
			padding: 1rem;
			display: flex;
			background-color: var(--sl-color-gray-50);
			border-bottom: 1px solid var(--sl-color-gray-200);
			align-items: center;
			justify-content: space-between;
		}

		sl-spinner {
			font-size: 4rem;
		}
	`;
}
