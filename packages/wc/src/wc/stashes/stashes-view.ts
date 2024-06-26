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
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/radio-button/radio-button.js';
import '@shoelace-style/shoelace/dist/components/radio-group/radio-group.js';
import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';
import { isStashTabError } from '@divicards/shared/error';
import { StashTabErrorsElement } from './e-stash-tab-errors';
import { ErrorLabel } from './types';
import { styles } from './stashes-view.styles';
import './e-stash-tab-container';
import { Task } from '@lit/task';
import { StashTabContainerElement } from './e-stash-tab-container';

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
	'upd:multiselect': boolean;
	'extract-cards': { tab: TabWithItems; league: League };

	'sample-from-tab': { sample: DivinationCardsSample; league: League; name: NoItemsTab['name'] };
	'tab-with-items-loaded': { tab: TabWithItems; league: League; name: string };
	tabs: NoItemsTab[];

	// ---
	/**  event from TabBadgeElement */
	'tab-select': { tabId: NoItemsTab['id']; name: NoItemsTab['name']; selected: boolean };
	'tab-click': { tabId: string; name: string };
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
	@property({ type: Boolean }) multiselect = false;

	@state() selectedTabs: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }> = new Map();
	@state() stashes: NoItemsTab[] = [];
	@state() noStashesMessage: string = '';
	@state() msg: string = '';
	@state() fetchingStashTab: boolean = false;
	@state() fetchingStash: boolean = false;
	@state() stashLoader!: IStashLoader;
	@state() errors: Array<ErrorLabel> = [];
	@state() stashLoadsAvailable = 30;
	@state() availableInTenSeconds = 15;
	@state() hoveredErrorTabId: string | null = null;
	@state() downloadedStashTabs: Array<TabWithItems> = [];
	@state() openedTabId: string | null = null;
	@state() openedTab: NoItemsTab | null = null;
	private stashTabTask = new Task(this, {
		task: async ([stashTabId]) => {
			if (!stashTabId) {
				return null;
			}
			return await this.#loadSingleTabContent(stashTabId, this.league, this.stashLoader.tab);
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
									: this.multiselect
									? html`<sl-button
											id="get-data-btn"
											class="btn-load-items"
											.disabled=${this.selectedTabs.size === 0 ||
											this.fetchingStashTab ||
											this.stashLoadsAvailable === 0}
											@click=${this.#onLoadItemsClicked}
									  >
											Load Tabs Contents
									  </sl-button>`
									: null}
							</div>
					  `
					: html`<div>
							${this.fetchingStash
								? html`<sl-button><sl-spinner></sl-spinner></sl-button>`
								: html`<sl-button id="stashes-btn" @click=${this.#loadStash}>Load Stash</sl-button>`}
					  </div> `}
				<div class="top-right-corner">
					${this.stashes.length
						? html`
								${this.multiselect
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
									  </sl-radio-group>`
									: null}
								<div class="tips">
									<wc-help-tip>
										<p>PoE API allows 30 requests in 5 minutes</p>
									</wc-help-tip>
									<div>Loads available: ${this.stashLoadsAvailable}</div>
								</div>
						  `
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
				.multiselect=${this.multiselect}
				league=${this.league}
				.stashes=${this.stashes}
				.selectedTabs=${this.selectedTabs}
				.errors=${this.errors}
				.hoveredErrorTabId=${this.hoveredErrorTabId}
				@upd:selectedTabs=${this.#onUpdSelectedTabs}
				@tab-click=${this.#onTabClick}
				@upd:multiselect=${this.#handleUpdMultiselect}
				.badgesDisabled=${this.stashLoadsAvailable === 0 || this.availableInTenSeconds === 0}
			></wc-tab-badge-group>
			${this.openedTabId
				? this.stashTabTask.render({
						pending: () => {
							return html`<e-stash-tab-container
								.badge=${this.openedTab}
								status="pending"
								@close=${this.#handleTabContainerClose}
							></e-stash-tab-container>`;
						},
						complete: tab =>
							html`<e-stash-tab-container
								@close=${this.#handleTabContainerClose}
								@extract-cards=${this.#emitExtractCards}
								.badge=${this.openedTab}
								status="complete"
								.tab=${tab}
							></e-stash-tab-container>`,
						initial: () => {
							return html`initial`;
						},
						error: (err: unknown) => {
							if (
								!(
									typeof err === 'object' &&
									err !== null &&
									'message' in err &&
									typeof err.message === 'string'
								)
							) {
								return;
							}
							return html`<div>${err.message}</div>`;
						},
				  })
				: null}
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
	#onTabClick(e: CustomEvent<Events['tab-click']>) {
		this.openedTabId = e.detail.tabId;
		this.openedTab = this.stashes.find(t => t.id === this.openedTabId) ?? null;
	}
	#handleUpdMultiselect(e: CustomEvent<boolean>) {
		this.multiselect = e.detail;
	}
	#emitExtractCards(e: Event) {
		const tab = (e.target as StashTabContainerElement)?.tab;
		if (tab) {
			this.emit<Events['extract-cards']>('extract-cards', { tab: tab, league: this.league });
		}
	}
	#handleTabContainerClose() {
		this.openedTab = null;
		this.openedTabId = null;
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

	async #waitForLoadsAvailable() {
		while (this.stashLoadsAvailable === 0 || this.availableInTenSeconds === 0) {
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
		}
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

		await this.#waitForLoadsAvailable();
		this.stashLoadsAvailable--;
		this.availableInTenSeconds--;
		setTimeout(() => {
			this.stashLoadsAvailable++;
		}, SECS_300);
		setTimeout(() => {
			this.availableInTenSeconds++;
		}, SECS_10);
		try {
			const singleTabContent = await loadFunction(id, league);
			return singleTabContent;
		} finally {
			// run again go clear wait-messages when time comes
			this.#waitForLoadsAvailable();
		}
	}
}

const sleepSecs = async (secs: number): Promise<void> => {
	return new Promise(r => setTimeout(r, secs * 1000));
};
