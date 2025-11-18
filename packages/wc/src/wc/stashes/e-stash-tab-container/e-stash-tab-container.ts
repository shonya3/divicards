import 'poe-custom-elements/stash-tab.js';
import '../poe-delve-priced-list';
import '../poe-simple-stash-tab';
import '../poe-map-stash-list';
import '../poe-currency-stash-list';
import '../poe-fragment-stash-list';
import '../poe-essence-stash-list';
import '../poe-gem-stash-list';
import '../poe-general-priced-list';
import '../poe-divination-stash-list';
import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import '@shoelace-style/shoelace/dist/components/copy-button/copy-button.js';
import '@shoelace-style/shoelace/dist/components/select/select.js';
import '@shoelace-style/shoelace/dist/components/option/option.js';
import '../e-tab-badge/e-tab-badge.js';
import '@shoelace-style/shoelace/dist/components/alert/alert.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
import '../../shared/e-json-viewer';

import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property, query } from 'lit/decorators.js';
import { TabWithItems } from 'poe-custom-elements/types.js';
import type { IStashLoader } from '@divicards/shared/IStashLoader.js';
import SlAlert from '@shoelace-style/shoelace/dist/components/alert/alert.js';
import { CloseEvent, ExtractCardsEvent } from './events.js';
import { SampleFromStashtabEvent } from '../events.js';

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
	@property({ type: Boolean }) cardsJustExtracted: boolean = false;
	@property() league: string = 'Standard';
    @property({ attribute: false }) stashLoader!: IStashLoader;
    @property({ type: Boolean }) viewJsonOpen: boolean = false;
    @property({ attribute: false }) divTabs: Array<{ id: string; name: string; type: string }> = [];
    @property() selectedDivTabId: string | null = null;
    @property({ type: Boolean }) extractingSelected: boolean = false;

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
                    ${this.tab ? html`<sl-button size="small" @click=${() => { this.viewJsonOpen = true; }}>View JSON</sl-button>` : null}
					</div>
                    ${this.tab && (this.tab.type as unknown as string) === 'DivinationCardStash' ? html`
                        <div class="div-controls">
                            <sl-select size="small" placeholder="Select Divination tab" .value=${this.selectedDivTabId ?? ''} @sl-change=${(e: any) => { this.selectedDivTabId = e.target.value; }}>
                                ${this.divTabs.map(t => html`<sl-option value=${t.id}>${t.name}</sl-option>`)}
                            </sl-select>
                            <sl-button size="small" variant="default" .disabled=${this.extractingSelected || !this.selectedDivTabId} @click=${this.#extractSelectedDivTab}>Extract selected tab</sl-button>
                        </div>
                    ` : null}
                    ${this.status === 'complete' && this.tab
                        ? this.tab.type === 'FragmentStash'
                            ? html` <sl-button @click=${this.#copyScarabs}>Copy Scarabs</sl-button> `
                            : (this.tab.type as unknown as string) === 'DivinationCardStash'
                                ? (this.cardsJustExtracted
                                    ? html`<sl-button variant="success">Extracted</sl-button>`
                                    : html`<sl-button @click=${this.#emitExtractCards}>Extract cards sample</sl-button>`)
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
			<sl-dialog label="Tab JSON" .open=${this.viewJsonOpen} @sl-hide=${() => { this.viewJsonOpen = false; }} style="--width: 800px;">
				${this.tab ? html`<e-json-viewer .data=${this.tab}></e-json-viewer>` : null}
				<sl-button slot="footer" variant="primary" @click=${() => { this.viewJsonOpen = false; }}>Close</sl-button>
			</sl-dialog>
			<div class="tab-box">
                ${this.tab && this.status === 'complete'
                    ? ((this.tab.type as unknown as string) === 'DelveStash' || stashtab_has_delve(this.tab))
                        ? html`<poe-delve-priced-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-delve-priced-list>`
                        : (this.tab.type as unknown as string) === 'MapStash'
                            ? html`<poe-map-stash-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-map-stash-list>`
                            : (this.tab.type as unknown as string) === 'CurrencyStash'
                                ? html`<poe-currency-stash-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-currency-stash-list>`
                        : (this.tab.type as unknown as string) === 'FragmentStash'
                            ? html`<poe-fragment-stash-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-fragment-stash-list>`
                            : (this.tab.type as unknown as string) === 'EssenceStash'
                                ? html`<poe-essence-stash-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-essence-stash-list>`
                                : (this.tab.type as unknown as string) === 'DivinationCardStash'
                                    ? html`<poe-divination-stash-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-divination-stash-list>`
                                : (this.tab.type as unknown as string) === 'GemStash'
                                    ? html`<poe-gem-stash-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-gem-stash-list>`
                                    : stashtab_has_gems(this.tab)
                                        ? html`<poe-gem-stash-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-gem-stash-list>`
                            : (this.tab.type as unknown as string) === 'PremiumStash' || (this.tab.type as unknown as string) === 'NormalStash' || (this.tab.type as unknown as string) === 'QuadStash'
                                ? html`<poe-general-priced-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-general-priced-list>`
                                : isSupportedTabType(this.tab.type)
                                    ? html`<poe-stash-tab .tab=${this.tab}></poe-stash-tab>`
                                    : html`<poe-general-priced-list .league=${this.league} .stashLoader=${this.stashLoader} .tab=${this.tab}></poe-general-priced-list>`
                : html`<sl-spinner></sl-spinner>`}
			</div> `;
		}

// legacy fallback renderer removed; unsupported types now route to <poe-simple-stash-tab>

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
		if (this.tab) {
			this.dispatchEvent(new ExtractCardsEvent(this.tab));
		} else {
			console.warn(`Tab is expected to be present but there is none`);
		}
	}
	#emitClose() {
		this.dispatchEvent(new CloseEvent());
	}

    async #loadDivTabs() {
        if (!this.stashLoader) return;
        try {
            const tabs = await this.stashLoader.tabs(this.league as any);
            this.divTabs = tabs.filter(t => (t as any).type === 'DivinationCardStash');
            if (!this.selectedDivTabId && this.divTabs.length) {
                this.selectedDivTabId = this.divTabs[0].id;
            }
        } catch (err) {
            console.warn('Failed to load tabs for div controls', err);
        }
    }

    async #extractSelectedDivTab() {
        if (!this.selectedDivTabId || this.extractingSelected) return;
        this.extractingSelected = true;
        try {
            const id = this.selectedDivTabId;
            const tab = this.divTabs.find(t => t.id === id);
            const sample = await this.stashLoader.sampleFromTab(id, this.league as any);
            const name = tab?.name || 'Div Tab';
            this.dispatchEvent(new SampleFromStashtabEvent(name, sample, this.league as any, { bubbles: true }));
        } catch (err) {
            console.error('Failed to extract selected div tab', err);
        }
        this.extractingSelected = false;
    }

    protected async firstUpdated(): Promise<void> {
        if (this.tab && (this.tab.type as unknown as string) === 'DivinationCardStash') {
            await this.#loadDivTabs();
        }
    }

    protected async willUpdate(map: Map<PropertyKey, unknown>): Promise<void> {
        if (map.has('league') || map.has('tab')) {
            if (this.tab && (this.tab.type as unknown as string) === 'DivinationCardStash') {
                await this.#loadDivTabs();
            }
        }
    }


	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

	:host {
		display: block;
		width: 100%;
		height: auto;
		border: 1px solid var(--sl-color-gray-200);
	}

	.tab-box {
		width: 100%;
		min-height: 480px;
		display: block;
	}

	.fallback {
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
		padding: 1rem;
		width: 100%;
		min-height: 480px;
		overflow: auto;
	}
		.fallback__title {
			color: var(--sl-color-danger-700);
			font-weight: 600;
		}
		.fallback__list {
			list-style: none;
			margin: 0;
			padding: 0;
			display: grid;
			grid-template-columns: 2fr 1fr;
			row-gap: 0.2rem;
			column-gap: 0.6rem;
		}

		.header-main {
			padding: 1rem;
			display: flex;
			background-color: var(--sl-color-gray-50);
			border-bottom: 1px solid var(--sl-color-gray-200);
			align-items: center;
			gap: 1rem;
			justify-content: space-between;
			min-height: 75px;
		}

		.badge-and-copy {
			display: flex;
			align-items: center;
			gap: 1rem;
		}

		.div-controls { display: flex; align-items: center; gap: 0.6rem; }

		sl-spinner {
			font-size: 4rem;
		}
	`;
}


function stashtab_has_gems(stashtab: TabWithItems | null): boolean {
    if (!stashtab) return false;
    const items = stashtab.items || [];
    for (const it of items) {
        const props = (it as any).properties || [];
        if (Array.isArray(props) && props.some((p: any) => p?.name === 'Gem Level' || p?.name === 'Level')) {
            return true;
        }
        const name = (it as any).typeLine || (it as any).baseType || (it as any).name || '';
        if (typeof name === 'string' && (name.includes(' Support') || name.includes('Gem'))) {
            return true;
        }
    }
    return false;
}

function stashtab_has_delve(stashtab: TabWithItems | null): boolean {
    if (!stashtab) return false;
    const items = stashtab.items || [];
    return items.some(it => {
        const s = ((it as any).baseType || (it as any).typeLine || (it as any).name || '') as string;
        const t = s.toLowerCase();
        if (t.includes('fossil') || t.includes('resonator')) return true;
        const icon = ((it as any).icon || '') as string;
        const ti = icon.toLowerCase();
        return ti.includes('fossil') || ti.includes('resonator');
    });
}

function isSupportedTabType(type: string): boolean {
    const supported = new Set([
        'PremiumStash',
        'CurrencyStash',
        'QuadStash',
        'FragmentStash',
        'EssenceStash',
        'GemStash',
        'Folder',
        'NormalStash',
        'DivinationCardStash',
    ]);
    return supported.has(type);
}

// legacy helper removed; routing sends unsupported types to <poe-simple-stash-tab>
