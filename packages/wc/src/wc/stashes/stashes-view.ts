import { IStashLoader } from '@divicards/shared/IStashLoader';
import { html, css, PropertyValues, nothing } from 'lit';
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
import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';
import { isStashTabError } from '@divicards/shared/error';
import { StashTabErrorsElement } from './e-stash-tab-errors';
import { ErrorLabel } from './types';
import { styles } from './stashes-view.styles';
import '../../../node_modules/poe-custom-elements/src/elements/poe-stash-tab';
import stash from '../../../node_modules/poe-custom-elements/jsons/QuadStashStd.json';
import { Task } from '@lit/task';

const SECS_300 = 300 * 1000;
const SECS_10 = 10 * 1000;

declare global {
	interface HTMLElementTagNameMap {
		'wc-stashes-view': StashesViewElement;
	}
}

export type Events = {
	close: void;
	// ---
	/** from tab-badge-group */
	'upd:selectedTabs': Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;
	/** from tab-badge-group */
	'upd:nameQuery': string;
	/** from tab-badge-group */
	'upd:PerPage': number;
	/** from tab-badge-group */
	'upd:page': number;

	'sample-from-tab': { sample: DivinationCardsSample; league: League; name: NoItemsTab['name'] };
	'tab-with-items-loaded': { tab: TabWithItems; league: League; name: string };
	tabs: NoItemsTab[];

	// ---
	/**  event from TabBadgeElement */
	'tab-select': { tabId: NoItemsTab['id']; name: NoItemsTab['name']; selected: boolean };
};

export interface StashesViewProps {
	league?: League;
	stashLoader: IStashLoader;
}

export type DownloadAs = (typeof DOWNLOAD_AS_VARIANTS)[number];
const DOWNLOAD_AS_VARIANTS = ['divination-cards-sample', 'general-tab'] as const;

export class StashesViewElement extends BaseElement {
	static override get defineList() {
		return [HelpTipElement, LeagueSelectElement, TabBadgeElement, TabBadgeGroupElement, StashTabErrorsElement];
	}
	static override tag = 'wc-stashes-view';
	static override styles = styles;

	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property() downloadAs: DownloadAs = 'divination-cards-sample';

	@state() selectedTabs: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }> = new Map();
	@state() stashes: NoItemsTab[] = [];
	@state() noStashesMessage: string = '';
	@state() msg: string = '';
	@state() fetchingStashTab: boolean = false;
	@state() fetchingStash: boolean = false;
	@state() stashLoader!: IStashLoader;
	@state() errors: Array<ErrorLabel> = [];
	@state() private stashLoadsAvailable = 30;
	@state() private availableInTenSeconds = 15;
	@state() hoveredErrorTabId: string | null = null;
	@state() downloadedStashTabs: Array<TabWithItems> = [];
	@state() openedTabId: string | null = null;
	private stashTabTask = new Task(this, {
		task: async ([stashTabId]) => {
			if (!stashTabId) {
				return;
			}

			const tab = await this.stashLoader.tab(stashTabId, this.league);
			this.downloadedStashTabs.push(tab);
			return tab;
		},
		args: () => [this.openedTabId],
	});

	@query('button#stashes-btn') stashesButton!: HTMLButtonElement;
	@query('button#get-data-btn') getDataButton!: HTMLButtonElement;

	protected willUpdate(map: PropertyValues<this>): void {
		if (map.has('league')) {
			this.stashes = [];
			this.msg = '';
			this.selectedTabs = new Map();
			this.errors = [];
		}
	}

	constructor() {
		super();
	}

	protected override render() {
		return html`<div class="main-stashes-component">
			<header class="header">
				<wc-league-select
					with-private-league-input
					.league=${this.league}
					@upd:league=${this.#onLeagueSelected}
				></wc-league-select>
				${this.stashes.length
					? html`
							<div>
								${this.fetchingStashTab
									? html`<sl-button><sl-spinner></sl-spinner></sl-button>`
									: html`<sl-button
											id="get-data-btn"
											class="btn-load-items"
											.disabled=${this.selectedTabs.size === 0 ||
											this.fetchingStashTab ||
											this.stashLoadsAvailable === 0}
											@click=${this.#onLoadItemsClicked}
									  >
											Load Tabs Contents
									  </sl-button>`}
							</div>
					  `
					: html`<div>
							${this.fetchingStash
								? html`<sl-button><sl-spinner></sl-spinner></sl-button>`
								: html`<sl-button id="stashes-btn" @click=${this.#loadStash}>Load Stash</sl-button>`}
					  </div> `}
				<div class="top-right-corner">
					${this.stashes.length
						? html`<sl-radio-group
									@sl-change=${this.#onDownloadAsChanged}
									.helpText=${`Download as`}
									value=${this.downloadAs}
								>
									${DOWNLOAD_AS_VARIANTS.map(
										variant =>
											html`<sl-radio-button size="small" value=${variant}
												>${variant === 'divination-cards-sample'
													? 'cards'
													: 'poe tab'}</sl-radio-button
											>`
									)}
								</sl-radio-group>
								<div class="tips">
									<wc-help-tip>
										<p>Select tabs by clicking on them. Then click LOAD ITEMS button</p>
										<p>PoE API allows 30 requests in 5 minutes</p>
									</wc-help-tip>
									<div>Loads available: ${this.stashLoadsAvailable}</div>
								</div> `
						: nothing}
					<sl-button @click=${this.#onCloseClicked} class="btn-close">Close</sl-button>
				</div>
			</header>
			<div class="messages">
				<p class="msg">${this.msg}</p>
				<p class="msg">${this.noStashesMessage}</p>
				${this.errors.length
					? html`<e-stash-tab-errors
							@upd:hoveredErrorTabId=${this.#handleUpdHoveredError}
							@upd:errors=${this.#handleUpdErrors}
							.errors=${this.errors}
					  ></e-stash-tab-errors>`
					: nothing}
			</div>
			<wc-tab-badge-group
				league=${this.league}
				.stashes=${this.stashes}
				.selectedTabs=${this.selectedTabs}
				.errors=${this.errors}
				.hoveredErrorTabId=${this.hoveredErrorTabId}
				@upd:selectedTabs=${this.#onUpdSelectedTabs}
				@tab-select=${this.#onTabSelect}
			></wc-tab-badge-group>
			${this.stashTabTask.render({
				pending: () => html`<sl-spinner></sl-spinner>`,
				complete: tab =>
					tab
						? html`<div class="stash-tab-container">
								<poe-stash-tab .tab=${tab}></poe-stash-tab>
						  </div>`
						: null,
			})}
		</div>`;
	}

	#handleUpdHoveredError(e: CustomEvent<string | null>) {
		this.hoveredErrorTabId = e.detail;
	}
	#handleUpdErrors(e: CustomEvent<Array<ErrorLabel>>) {
		this.errors = e.detail;
	}
	#onLoadItemsClicked() {
		this.#loadSelectedTabs(this.league);
	}
	#onCloseClicked() {
		this.emit('close');
	}
	#onDownloadAsChanged(e: InputEvent) {
		this.downloadAs = (e.target as HTMLInputElement).value as DownloadAs;
	}
	#onLeagueSelected(e: CustomEvent<League>) {
		this.league = e.detail;
	}
	#onUpdSelectedTabs(e: CustomEvent<Events['upd:selectedTabs']>) {
		const map = (e as CustomEvent<Events['upd:selectedTabs']>).detail;
		this.selectedTabs = new Map(map);
	}
	#onTabSelect(e: CustomEvent<Events['tab-select']>) {
		if (e.detail.selected) {
			this.openedTabId = e.detail.tabId;
		}
	}

	/** Load whole stash of Array<NoItemsTab> and emits it  */
	async #loadStash() {
		if (!this.stashLoader) {
			throw new Error('No stash loader');
		}
		this.noStashesMessage = '';
		this.fetchingStash = true;
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
		} finally {
			this.fetchingStash = false;
		}
	}

	/** For each tab, loads sample and emits it */
	async #loadSelectedTabs(league: League): Promise<void> {
		while (this.selectedTabs.size > 0) {
			for (const { id, name } of this.selectedTabs.values()) {
				this.fetchingStashTab = true;
				try {
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
				} catch (err) {
					if (!isStashTabError(err)) {
						throw err;
					}
					const noItemsTab = this.stashes.find(stash => stash.id === id);
					if (noItemsTab) {
						this.errors = [...this.errors, { noItemsTab, message: err.message }];
					}
				} finally {
					this.selectedTabs.delete(id);
					this.selectedTabs = new Map(this.selectedTabs);
					this.fetchingStashTab = false;
					this.msg = '';
				}
			}
		}
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
		try {
			const singleTabContent = await loadFunction(id, league);
			return singleTabContent;
		} finally {
			this.stashLoadsAvailable--;
			this.availableInTenSeconds--;
			setTimeout(() => {
				this.stashLoadsAvailable++;
			}, SECS_300);
			setTimeout(() => {
				this.availableInTenSeconds++;
			}, SECS_10);
		}
	}
}

const sleepSecs = async (secs: number): Promise<void> => {
	return new Promise(r => setTimeout(r, secs * 1000));
};
