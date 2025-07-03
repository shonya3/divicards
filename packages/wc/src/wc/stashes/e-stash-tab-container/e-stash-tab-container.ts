import 'poe-custom-elements/stash-tab.js';
import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import '@shoelace-style/shoelace/dist/components/copy-button/copy-button.js';
import '../e-tab-badge/e-tab-badge.js';
import '@shoelace-style/shoelace/dist/components/alert/alert.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';

import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property, query } from 'lit/decorators.js';
import { TabWithItems } from 'poe-custom-elements/types.js';
import SlAlert from '@shoelace-style/shoelace/dist/components/alert/alert.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-stash-tab-container': StashTabContainerElement;
	}
}

/**
 * @summary Container for poe stash tab with header with actions.
 * @event e-stash-tab-container__extract-cards - Emitted on "Extract cards sample" button click.
 * @event e-stash-tab-container__close - Emitted on "X" button click.
 */
@customElement('e-stash-tab-container')
export class StashTabContainerElement extends LitElement {
	/** PoE API tab data https://www.pathofexile.com/developer/docs/reference#stashes-get */
	@property({ type: Object }) tab: TabWithItems | null = null;
	@property() status: 'pending' | 'complete' = 'pending';

	@query('sl-alert') scarabsSuccessAlert!: SlAlert;

	protected render(): TemplateResult {
		return html`<header class="header">
				<div class="header-main">
					<div class="badge-and-copy">
						${this.tab ? html`<e-tab-badge as="button" .tab=${this.tab}></e-tab-badge>` : null}
						${this.tab
							? html`<sl-copy-button
									.value=${JSON.stringify(this.tab, null, 4)}
									.copyLabel=${`Click to copy JSON of the tab`}
									.successLabel=${`You copied JSON of the tab`}
									.errorLabel=${`Whoops, your browser doesn't support this!`}
							  ></sl-copy-button>`
							: null}
					</div>
					${this.status === 'complete' && this.tab
						? this.tab.type === 'FragmentStash'
							? html` <sl-button @click=${this.#copyScarabs}>Copy Scarabs</sl-button> `
							: stashtab_has_cards(this.tab)
							? html`<sl-button @click=${this.#emitExtractCards}>Extract cards sample</sl-button>`
							: null
						: null}
					<sl-icon-button name="x-lg" @click=${this.#emitClose} class="btn-close">X</sl-icon-button>
				</div>
				<div class="alerts">
					<sl-alert variant="success" duration="2000" closable>
						<sl-icon slot="icon" name="info-circle"></sl-icon>
						Scarabs copied to your clipboard!
					</sl-alert>
				</div>
			</header>
			<div class="tab-box">
				${this.tab && this.status === 'complete'
					? html`<poe-stash-tab .tab=${this.tab}></poe-stash-tab>`
					: html`<sl-spinner></sl-spinner>`}
			</div> `;
	}

	#copyScarabs() {
		if (!this.tab) {
			console.error('Cannot extract scarabs because there is no tab data');
			return;
		}
		const s = this.tab.items
			.filter(item => item.baseType.includes('Scarab'))
			.sort((a, b) => (b.stackSize ?? 0) - (a.stackSize ?? 0))
			.map(scarab => `${scarab.baseType},${scarab.stackSize}`)
			.join('\n');
		navigator.clipboard.writeText(s).then(() => {
			this.scarabsSuccessAlert.show();
		});
	}
	#emitExtractCards() {
		this.dispatchEvent(new Event('extract-cards'));
	}
	#emitClose() {
		this.dispatchEvent(new Event('close'));
	}

	static styles: CSSResult = css`
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

		.header-main {
			padding: 1rem;
			display: flex;
			background-color: var(--sl-color-gray-50);
			border-bottom: 1px solid var(--sl-color-gray-200);
			align-items: center;
			justify-content: space-between;
			min-height: 75px;
		}

		.badge-and-copy {
			display: flex;
			align-items: center;
			gap: 1rem;
		}

		sl-spinner {
			font-size: 4rem;
		}
	`;
}

function stashtab_has_cards(stashtab: TabWithItems | null): boolean {
	if (!stashtab) {
		return false;
	}

	return stashtab.items.some(item => item.frameType === 6);
}
