import { IDefaultStashLoader } from '@divicards/shared/IStashLoader';
import { html, css, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import '../e-help-tip';
import { TabBadgeElement } from './tab-badge';
import { LeagueSelectElement } from '../league-select';
import { property, state, query } from 'lit/decorators.js';
import { League } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { TabBadgeGroupElement } from './tab-badge-group';
import { classMap } from 'lit/directives/class-map.js';
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';

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
		'e-stash-loader': StashLoaderElement;
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

	tab: { tab: TabWithItems; league: League; name: NoItemsTab['name'] };
	tabs: NoItemsTab[];

	// ---
	/**  event from TabBadgeElement */
	'tab-select': { tabId: NoItemsTab['id']; name: NoItemsTab['name']; selected: boolean };
};

export interface Props {
	league?: League;
	stashLoader: IDefaultStashLoader;
}

export class StashLoaderElement extends BaseElement {
	static override get defineList() {
		return [LeagueSelectElement, TabBadgeElement, TabBadgeGroupElement];
	}
	static override tag = 'e-stash-loader';
	static override styles = [styles()];

	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property() customLeague: string = CustomLeagueStorage.load() ?? '';

	@state() selectedTabs: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }> = new Map();
	@state() stashes: NoItemsTab[] = [];
	@state() noStashesMessage: string = '';
	@state() msg: string = '';
	@state() fetchingStash: boolean = false;
	@state() stashLoader!: IDefaultStashLoader;
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

	async #onLoadItemsClicked() {
		await this.loadSelectedTabs(this.league);
	}

	#onCloseClicked() {
		this.emit('close');
	}

	async #onLoadStashesList() {
		if (!this.stashLoader) {
			throw new Error('No stash loader');
		}
		this.noStashesMessage = '';
		try {
			this.stashes = await this.stashLoader.tabs(this.league);
			this.emit<Events['tabs']>('tabs', this.stashes);
			this.stashes;
			if (!this.stashes.length) {
				this.noStashesMessage = 'No stashes here. Try to change the league';
			}
		} catch (err) {
			console.dir(err);
			if (err instanceof Error) {
				this.noStashesMessage = err.message;
			} else if (typeof err === 'string') {
				this.noStashesMessage = err;
			}
			throw err;
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

	protected override render() {
		return html`<div class="main-stashes-component">
			<div class="controls">
				<div class="league-stashes">
					<wc-league-select .league=${this.league} @upd:league=${this.#onLeagueSelected}></wc-league-select>
					<div class="custom-league">
						<label for="custom-league-input">Custom league (for private leagues)</label>
						<input
							.value=${this.customLeague}
							@input=${this.#onCustomLeagueInput}
							id="custom-league-input"
							type="text"
						/>
					</div>
					<button id="stashes-btn" @click=${this.#onLoadStashesList}>Stashes</button>
					<e-help-tip>
						<p>Select tabs by clicking on them. Then click LOAD ITEMS button</p>
					</e-help-tip>
					<div>Loads available: ${this.stashLoadsAvailable}</div>
				</div>

				<button
					id="get-data-btn"
					class=${classMap({ 'not-visible': this.selectedTabs.size === 0, 'btn-load-items': true })}
					.disabled=${this.selectedTabs.size === 0 || this.fetchingStash || this.stashLoadsAvailable === 0}
					@click=${this.#onLoadItemsClicked}
				>
					load items
				</button>

				<button @click=${this.#onCloseClicked} class="btn-close">Close</button>
			</div>

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
					const tab = await this.#loadSample(id, league);
					this.emit<Events['tab']>('tab', { name, tab, league });
				}
			} catch (err) {
				await this.#handleLoadTabError(err);
			}
		}

		this.fetchingStash = false;
		this.msg = '';
	}

	async #loadSample(id: string, league: League): Promise<TabWithItems> {
		if (!this.stashLoader) {
			throw new Error('No stash loader');
		}

		this.msg = '';

		const tab = await this.stashLoader.tab(league, id);
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

		return tab;
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
			border: 2px solid #000;
			border-radius: 0.25rem;
		}

		.btn-load-items {
			border: 2px solid transparent;
			text-transform: uppercase;
		}

		.btn-load-items:not(:disabled) {
			transform: scale(1.25);
			border-color: purple;
		}

		.league-stashes {
			max-width: max-content;
			display: flex;
			align-items: center;
			gap: 1rem;
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

		.controls {
			display: flex;
			justify-content: space-between;
		}
	`;
}
