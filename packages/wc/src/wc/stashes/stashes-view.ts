import { IStashLoader } from '@divicards/shared/IStashLoader';
import { html, css, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import { HelpTipElement } from '../help-tip';
import { TabBadgeElement } from './tab-badge';
import { LeagueSelectElement } from '../league-select';
import { property, state, query } from 'lit/decorators.js';
import { NoItemsTab } from '@divicards/shared/poe.types';
import { DivinationCardsSample, League } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { TabBadgeGroupElement } from './tab-badge-group';
import { classMap } from 'lit/directives/class-map.js';

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

	// ---
	/**  event from TabBadgeElement */
	'tab-select': { tabId: TabBadgeElement['tabId']; name: TabBadgeElement['name']; selected: boolean };
}

export interface StashesViewProps {
	league?: League;
	stashLoader: IStashLoader;
}

export class StashesViewElement extends BaseElement {
	static override get defineList() {
		return [HelpTipElement, LeagueSelectElement, TabBadgeElement, TabBadgeGroupElement];
	}
	static override tag = 'wc-stashes-view';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property() customLeague: string = CustomLeagueStorage.load() ?? '';

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
			console.log(this.customLeague);
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
		this.stashes = await this.stashLoader.tabs(this.league);
		this.stashes;
		if (!this.stashes.length) {
			this.noStashesMessage = 'No stashes here. Try to change the league';
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
					<wc-help-tip>
						<p>Select tabs by clicking on them. Then click LOAD ITEMS button</p>
					</wc-help-tip>
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
					const sample = await this.#loadSample(id, name, league);
					this.emit<Events['sample-from-tab']>('sample-from-tab', { name, sample, league });
				}
			} catch (err) {
				this.#handleLoadTabError(err);
			}
		}

		this.fetchingStash = false;
		this.msg = '';
	}

	async #loadSample(id: string, name: string, league: League): Promise<DivinationCardsSample> {
		if (!this.stashLoader) {
			throw new Error('No stash loader');
		}

		this.msg = '';

		const sample = await this.stashLoader.sampleFromTab(id, league);
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

		return sample;
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
