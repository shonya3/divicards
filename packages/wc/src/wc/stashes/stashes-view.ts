import { IStashLoader } from '@divicards/shared/IStashLoader';
import { html, css, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import { HelpTipElement } from '../help-tip';
import { TabBadgeElement } from './tab-badge';
import { LeagueSelectElement } from '../league-select';
import { property, state, query } from 'lit/decorators.js';
import { NoItemsTab, TabWithItems } from '@divicards/shared/poe.types';
import { DivinationCardsSample, League } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { TabBadgeGroupElement } from './tab-badge-group';
import { classMap } from 'lit/directives/class-map.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/radio-button/radio-button.js';
import '@shoelace-style/shoelace/dist/components/radio-group/radio-group.js';

class CustomLeagueStorage {
	static #key = 'CUSTOM_LEAGUE';
	static save(s: string) {
		localStorage.setItem(this.#key, s);
	}

	static load(): string | null {
		return localStorage.getItem(this.#key);
	}
}

const SECS_300 = 300 * 1000;
const SECS_10 = 10 * 1000;
const sleepSecs = async (secs: number): Promise<void> => {
	return new Promise(r => setTimeout(r, secs * 1000));
};

declare global {
	interface HTMLElementTagNameMap {
		'wc-stashes-view': StashesViewElement;
	}
}

export class Events {
	'close': void;
	// ---
	/** from tab-badge-group */
	'upd:selectedTabs': Map<TabBadgeElement['tabId'], { id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }>;
	/** from tab-badge-group */
	'upd:nameQuery': string;
	/** from tab-badge-group */
	'upd:PerPage': number;
	/** from tab-badge-group */
	'upd:page': number;

	'sample-from-tab': { sample: DivinationCardsSample; league: League; name: TabBadgeElement['name'] };
	'tab-with-items-loaded': { tab: TabWithItems; league: League; name: string };
	'tabs': NoItemsTab[];

	// ---
	/**  event from TabBadgeElement */
	'tab-select': { tabId: TabBadgeElement['tabId']; name: TabBadgeElement['name']; selected: boolean };
}

export interface StashesViewProps {
	league?: League;
	stashLoader: IStashLoader;
}

type DownloadAs = (typeof DOWNLOAD_AS_VARIANTS)[number];
const DOWNLOAD_AS_VARIANTS = ['divination-cards-sample', 'general-tab'] as const;

export class StashesViewElement extends BaseElement {
	static override get defineList() {
		return [HelpTipElement, LeagueSelectElement, TabBadgeElement, TabBadgeGroupElement];
	}
	static override tag = 'wc-stashes-view';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property() customLeague: string = CustomLeagueStorage.load() ?? '';
	@property() downloadAs: DownloadAs = 'divination-cards-sample';

	@state() selectedTabs: Map<
		TabBadgeElement['tabId'],
		{ id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }
	> = new Map();
	@state() stashes: NoItemsTab[] = [];
	@state() noStashesMessage: string = '';
	@state() msg: string = '';
	@state() fetchingStash: boolean = false;
	@state() stashLoader!: IStashLoader;
	@state() private stashLoadsAvailable = 30;
	@state() private availableInTenSeconds = 15;

	@query('button#stashes-btn') stashesButton!: HTMLButtonElement;
	@query('button#get-data-btn') getDataButton!: HTMLButtonElement;

	protected willUpdate(map: PropertyValues<this>): void {
		if (map.has('league')) {
			this.stashes = [];
			this.msg = '';
			this.selectedTabs = new Map();
		}

		if (map.has('customLeague')) {
			CustomLeagueStorage.save(this.customLeague);
			if (this.customLeague) {
				this.league = this.customLeague;
			}
		}
	}

	protected override render() {
		return html`<div class="main-stashes-component">
			<div class="top-right-corner">
				<div class=${classMap({ tips: true, hidden: this.stashes.length === 0 })}>
					<wc-help-tip>
						<p>Select tabs by clicking on them. Then click LOAD ITEMS button</p>
					</wc-help-tip>
					<div>Loads available: ${this.stashLoadsAvailable}</div>
				</div>
				<sl-button @click=${this.#onCloseClicked} class="btn-close">Close</sl-button>
			</div>

			<div class="controls">
				<fieldset>
					<legend>Choose league</legend>
					<wc-league-select .league=${this.league} @upd:league=${this.#onLeagueSelected}></wc-league-select>

					<sl-input
						.value=${this.customLeague}
						@sl-input=${this.#onCustomLeagueInput}
						id="custom-league-input"
						type="text"
						label="Custom league (for private leagues)"
					></sl-input>

					<sl-radio-group
						class="mt-16"
						@sl-change=${this.#onDownloadAsChanged}
						label="Download as"
						value=${this.downloadAs}
					>
						${DOWNLOAD_AS_VARIANTS.map(
							variant => html`<sl-radio-button value=${variant}>${variant}</sl-radio-button>`
						)}
					</sl-radio-group>
				</fieldset>
				<div class="load-stashes-section">
					<sl-button id="stashes-btn" @click=${this.#onLoadStashesList}>Load Stash</sl-button>
				</div>
			</div>

			<sl-button
				id="get-data-btn"
				class=${classMap({ 'not-visible': this.stashes.length === 0, 'btn-load-items': true })}
				.disabled=${this.selectedTabs.size === 0 || this.fetchingStash || this.stashLoadsAvailable === 0}
				@click=${this.#onLoadItemsClicked}
			>
				Load Tabs Contents
			</sl-button>

			<div class="messages">
				<p class=${classMap({ visible: this.msg.length > 0, msg: true })}>${this.msg}</p>
				<p class=${classMap({ visible: this.noStashesMessage.length > 0, msg: true })}>
					${this.noStashesMessage}
				</p>
			</div>

			<wc-tab-badge-group
				league=${this.league}
				.stashes=${this.stashes}
				.selectedTabs=${this.selectedTabs}
				@upd:selectedTabs=${this.#onUpdSelectedTabs}
			></wc-tab-badge-group>
		</div>`;
	}

	async #onLoadItemsClicked() {
		await this.loadSelectedTabs(this.league);
	}

	#onCloseClicked() {
		this.emit('close');
	}

	#onDownloadAsChanged(e: InputEvent) {
		this.downloadAs = (e.target as HTMLInputElement).value as DownloadAs;
	}

	async #onLoadStashesList() {
		if (!this.stashLoader) {
			throw new Error('No stash loader');
		}
		this.noStashesMessage = '';

		try {
			this.stashes = await this.stashLoader.tabs(this.league);
			this.emit<Events['tabs']>('tabs', this.stashes);
			if (!this.stashes.length) {
				this.noStashesMessage = 'No stashes here. Try to change the league';
			}
		} catch (err) {
			if (err instanceof Error) {
				this.noStashesMessage = err.message;
			} else if (typeof err === 'string') {
				this.noStashesMessage = err;
			} else {
				throw err;
			}
		}
	}

	#onLeagueSelected(e: CustomEvent<League>) {
		this.league = e.detail;
	}

	#onUpdSelectedTabs(e: CustomEvent<Events['upd:selectedTabs']>) {
		const map = (e as CustomEvent<Events['upd:selectedTabs']>).detail;
		this.selectedTabs = new Map(map);
	}

	#onCustomLeagueInput(e: InputEvent) {
		const target = e.target as HTMLInputElement;
		this.customLeague = target.value;
	}

	/**
	 * For each tab, loads sample and emits it
	 */
	protected async loadSelectedTabs(league: League): Promise<void> {
		this.fetchingStash = true;
		while (this.selectedTabs.size > 0) {
			try {
				for (const { id, name } of this.selectedTabs.values()) {
					if (this.stashLoadsAvailable === 0) {
						this.msg = 'Loads available: 0. Waiting for cooldown.';
						await sleepSecs(1);
						continue;
					}
					if (this.availableInTenSeconds === 0) {
						this.msg = 'Sleep for short cooldown';
						await sleepSecs(0.5);
						continue;
					}
					switch (this.downloadAs) {
						case 'divination-cards-sample': {
							const sample = await this.#loadSingleTabContent(id, league, this.stashLoader.sampleFromTab);
							this.emit<Events['sample-from-tab']>('sample-from-tab', {
								name,
								sample,
								league,
							});

							break;
						}
						case 'general-tab': {
							const tab = await this.#loadSingleTabContent(id, league, this.stashLoader.tab);
							tab.name = name;
							this.emit<Events['tab-with-items-loaded']>('tab-with-items-loaded', { tab, name, league });

							break;
						}
					}
				}
			} catch (err) {
				await this.#handleLoadTabError(err);
			}
		}

		this.fetchingStash = false;
		this.msg = '';
	}

	async #loadSingleTabContent<T>(
		id: string,
		league: League,
		loadFunction: (id: string, league: League) => T
	): Promise<T> {
		if (!this.stashLoader) {
			throw new Error('No stash loader');
		}

		this.msg = '';

		// const loadFunction = mode === 'sample' ? this.stashLoader.sampleFromTab : this.stashLoader.tab;
		const singleTabContent = await loadFunction(id, league);
		this.selectedTabs.delete(id);
		this.selectedTabs = new Map(this.selectedTabs);

		this.stashLoadsAvailable--;
		setTimeout(() => {
			this.stashLoadsAvailable++;
		}, SECS_300);

		this.availableInTenSeconds--;
		setTimeout(() => {
			this.availableInTenSeconds++;
		}, SECS_10);

		return singleTabContent;
	}

	async #handleLoadTabError(err: unknown): Promise<void> {
		if (typeof err === 'object' && err !== null && 'message' in err) {
			if (typeof err.message === 'string') {
				this.msg = err.message;
			}
		}
		this.fetchingStash = false;
		this.selectedTabs = new Map();
		throw err;
	}
}

function styles() {
	return css`
		.main-stashes-component {
			position: relative;
			padding: 1rem;
			border: 2px solid rgba(0, 0, 0, 0.6);
			border-radius: 0.25rem;
		}

		wc-help-tip::part(tooltip) {
			right: 5px;
		}

		.controls {
			display: flex;
			max-width: 60%;
			justify-content: space-between;
		}

		.tips {
			display: flex;
			gap: 0.5rem;
		}

		.top-right-corner {
			position: absolute;
			top: 1rem;
			right: 1rem;
			display: flex;
			align-items: center;
			gap: 2rem;
		}

		.btn-close {
			margin-left: auto;
		}

		.btn-load-items {
			text-transform: uppercase;
			margin: auto;
			display: block;
			width: fit-content;
			margin-top: 0.4rem;
		}

		.btn-load-items:not(:disabled) {
			transform: scale(1.45);
		}

		.messages {
			position: relative;
			height: 4rem;
		}

		.msg {
			position: absolute;
			font-size: 2rem;
			max-width: max-content;
			margin-inline: auto;
			visibility: hidden;
			margin-block: 0;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
		}

		.visible {
			visibility: visible;
		}

		.not-visible {
			visibility: hidden;
		}

		.hidden {
			display: none;
		}

		.mt-16 {
			margin-top: 1rem;
		}

		fieldset {
			border: none;
		}
	`;
}
