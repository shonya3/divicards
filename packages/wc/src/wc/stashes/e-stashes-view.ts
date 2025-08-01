import type { IStashLoader } from '@divicards/shared/IStashLoader.js';
import { html, PropertyValues, nothing, LitElement, CSSResult, TemplateResult } from 'lit';
import '../e-help-tip';
import '../e-league-select';
import './e-tab-badge-group/e-tab-badge-group.js';
import './e-stash-tab-errors';
import { property, state, query, customElement } from 'lit/decorators.js';
import { type League } from '@divicards/shared/types.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/radio-button/radio-button.js';
import '@shoelace-style/shoelace/dist/components/radio-group/radio-group.js';
import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';
import { isStashTabError } from '@divicards/shared/error.js';
import type { ErrorLabel, SelectedStashtabs } from './types.js';
import { styles } from './e-stashes-view.styles.js';
import './e-stash-tab-container/e-stash-tab-container.js';
import { Task } from '@lit/task';
import { ExtractCardsEvent as ContainerExtractCardsEvent } from './e-stash-tab-container/events.js';
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';
import { LeagueChangeEvent } from '../events/change/league.js';
import {
	ExtractCardsEvent,
	SampleFromStashtabEvent,
	SelectedTabsChangeEvent,
	StashtabFetchedEvent,
	StashtabsBadgesFetchedEvent,
	CloseEvent,
	Events,
} from './events.js';
import { DefineComponent } from 'vue';
import { VueEventHandlers } from '../../event-utils.js';
import { MultiselectChangeEvent } from './e-tab-badge-group/events.js';
import { TabClickEvent } from './e-tab-badge/events.js';

const SECS_300 = 300 * 1000;
const SECS_10 = 10 * 1000;

export interface StashesViewProps {
	league?: League;
	stashLoader: IStashLoader;
}

export type DownloadAs = (typeof DOWNLOAD_AS_VARIANTS)[number];
const DOWNLOAD_AS_VARIANTS = ['divination-cards-sample', 'general-tab'] as const;

@customElement('e-stashes-view')
export class StashesViewElement extends LitElement {
	static override styles: Array<CSSResult> = [styles];

	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property() downloadAs: DownloadAs = 'divination-cards-sample';
	@property({ type: Boolean }) multiselect = false;

	@state() selected_tabs: SelectedStashtabs = new Map();
	/** PoE /stashes data about all stashtabs in stash (does not include items) */
	@state() stashtabs_badges: NoItemsTab[] = [];
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
	@state() opened_tab: NoItemsTab | null = null;
	/** Indicator whether cards was just extracted. */
	@state() cardsJustExtracted = false;
	private stashTabTask = new Task(this, {
		task: async ([tab]) => {
			if (!tab) {
				return null;
			}
			return await this.#loadSingleTabContent(tab.id, this.league, this.stashLoader.tab);
		},
		args: () => [this.opened_tab],
	});

	constructor() {
		super();

		this.addEventListener('stashes__tab-click', e => {
			this.#handle_tab_badge_click(e);
			e.stopPropagation();
		});
	}

	@query('button#stashes-btn') stashesButton!: HTMLButtonElement;
	@query('button#get-data-btn') getDataButton!: HTMLButtonElement;

	protected willUpdate(map: PropertyValues<this>): void {
		if (map.has('league')) {
			this.stashtabs_badges = [];
			this.msg = '';
			this.selected_tabs = new Map();
			this.errors = [];
		}
	}

	protected override render(): TemplateResult {
		return html`<div class="main-stashes-component">
			<header class="header">
				<e-league-select
					with-private-league-input
					.league=${this.league}
					@change:league=${this.#handle_league_selected}
				></e-league-select>
				${this.stashtabs_badges.length
					? html`
							<div>
								${this.fetchingStashTab
									? html`<sl-button><sl-spinner></sl-spinner></sl-button>`
									: this.multiselect
									? html`<sl-button
											id="get-data-btn"
											class="btn-load-items"
											.disabled=${this.selected_tabs.size === 0 ||
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
								? html`<sl-button size="small"><sl-spinner></sl-spinner></sl-button>`
								: html`<sl-button
										variant="primary"
										size="small"
										id="stashes-btn"
										@click=${this.#loadStash}
										>Load Stash</sl-button
								  >`}
					  </div> `}
				<div class="top-right-corner">
					${this.stashtabs_badges.length
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
									<e-help-tip>
										<p>PoE API allows 30 requests in 5 minutes</p>
									</e-help-tip>
									<div class="loads-available">
										Loads available:<span class="loads-available__value"
											>${this.stashLoadsAvailable}</span
										>
									</div>
								</div>
						  `
						: nothing}
					<sl-button size="small" @click=${this.#onCloseClicked} class="btn-close">Close</sl-button>
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
			<e-tab-badge-group
				.multiselect=${this.multiselect}
				league=${this.league}
				.stashes=${this.stashtabs_badges}
				.selected_tabs=${this.selected_tabs}
				.errors=${this.errors}
				.hoveredErrorTabId=${this.hoveredErrorTabId}
				@change:multiselect=${this.#change_multiselect}
				@change:selected_tabs=${this.#handle_selected_tabs_change}
				.badgesDisabled=${this.stashLoadsAvailable === 0 || this.availableInTenSeconds === 0}
			></e-tab-badge-group>
			${this.opened_tab
				? this.stashTabTask.render({
						pending: () => {
							return html`<e-stash-tab-container
								status="pending"
								@e-stash-tab-container__close=${this.#handleTabContainerClose}
							></e-stash-tab-container>`;
						},
						complete: tab =>
							html`<e-stash-tab-container
								.cardsJustExtracted=${this.cardsJustExtracted}
								@e-stash-tab-container__close=${this.#handleTabContainerClose}
								@e-stash-tab-container__extract-cards=${this.#emitExtractCards}
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
		this.#load_selected_tabs(this.league);
	}
	#onCloseClicked() {
		this.dispatchEvent(new CloseEvent());
	}
	#onDownloadAsChanged(e: InputEvent) {
		this.downloadAs = (e.target as HTMLInputElement).value as DownloadAs;
	}
	#handle_league_selected(e: LeagueChangeEvent): void {
		this.league = e.$league;
		this.dispatchEvent(new LeagueChangeEvent(e.$league));
	}
	#handle_selected_tabs_change(e: SelectedTabsChangeEvent): void {
		this.selected_tabs = new Map(e.$selected_tabs);
		this.dispatchEvent(new SelectedTabsChangeEvent(this.selected_tabs));
	}
	#handle_tab_badge_click(e: TabClickEvent): void {
		this.opened_tab = e.$tab;
	}
	#change_multiselect(e: MultiselectChangeEvent): void {
		this.multiselect = e.$multiselect;
	}
	#emitExtractCards(e: ContainerExtractCardsEvent) {
		this.cardsJustExtracted = true;
		setTimeout(() => {
			this.cardsJustExtracted = false;
		}, 2000);
		this.dispatchEvent(new ExtractCardsEvent(e.$tab, this.league));
	}
	#handleTabContainerClose() {
		this.opened_tab = null;
	}

	/** Load whole stash of Array<NoItemsTab> and emits it  */
	async #loadStash() {
		if (!this.stashLoader) {
			throw new Error('No stash loader');
		}
		this.noStashesMessage = '';
		this.fetchingStash = true;
		try {
			this.stashtabs_badges = await this.stashLoader.tabs(this.league);
			this.dispatchEvent(new StashtabsBadgesFetchedEvent(this.stashtabs_badges));
			if (!this.stashtabs_badges.length) {
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

	/** For each selected stashtab badge, load stashtab and emit it */
	async #load_selected_tabs(league: League): Promise<void> {
		while (this.selected_tabs.size > 0) {
			for (const { id, name: stashtab_name } of this.selected_tabs.values()) {
				this.fetchingStashTab = true;
				try {
					switch (this.downloadAs) {
						case 'divination-cards-sample': {
							const sample = await this.#loadSingleTabContent(id, league, this.stashLoader.sampleFromTab);
							this.dispatchEvent(new SampleFromStashtabEvent(stashtab_name, sample, league));
							break;
						}
						case 'general-tab': {
							const stashtab = await this.#loadSingleTabContent(id, league, this.stashLoader.tab);
							this.dispatchEvent(new StashtabFetchedEvent(stashtab, this.league));
							break;
						}
					}
				} catch (err) {
					if (!isStashTabError(err)) {
						throw err;
					}
					const stashtab_badge = this.stashtabs_badges.find(stash => stash.id === id);
					if (stashtab_badge) {
						this.errors = [
							...this.errors,
							{
								noItemsTab: stashtab_badge,
								message: err.message,
							},
						];
					}
				} finally {
					this.selected_tabs.delete(id);
					this.selected_tabs = new Map(this.selected_tabs);
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

declare global {
	interface HTMLElementTagNameMap {
		'e-stashes-view': StashesViewElement;
	}
}

declare module 'vue' {
	interface GlobalComponents {
		'e-stashes-view': DefineComponent<StashesViewProps & VueEventHandlers<Events>>;
	}
}
