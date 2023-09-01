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

const SECS_300 = 300 * 1000;

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
	static override defineList = [HelpTipElement, LeagueSelectElement, TabBadgeElement, TabBadgeGroupElement];
	static override tag = 'wc-stashes-view';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;

	#countdownTimer: ReturnType<typeof setInterval> | null = null;
	@state() selectedTabs: Map<
		TabBadgeElement['tabId'],
		{ id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }
	> = new Map();
	@state() stashes: NoItemsTab[] = [];
	@state() noStashesMessage: string = '';
	@state() msg: string = '';
	@state() fetchingStash: boolean = false;
	@state() countdown: number = 0;
	@state() stashLoader!: IStashLoader;
	@state() stashLoadsAvailable = 30;

	@query('button#stashes-btn') stashesButton!: HTMLButtonElement;
	@query('button#get-data-btn') getDataButton!: HTMLButtonElement;

	protected willUpdate(map: PropertyValues<this>): void {
		if (map.has('league')) {
			this.stashes = [];
			this.msg = '';
		}
	}

	async #onLoadItemsClicked() {
		await this.fetchStashesContents(this.league);
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

	render() {
		return html`<div class="main-stashes-component">
			<div class="controls">
				<div class="league-stashes">
					<wc-league-select .league=${this.league} @upd:league=${this.#onLeagueSelected}></wc-league-select>
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

	async fetchStashesContents(league: League) {
		const SLEEP_SECS = 10;
		const LOAD_AT_ONE_ITERATION = 5;
		this.fetchingStash = true;
		while (this.selectedTabs.size > 0) {
			if (this.stashLoadsAvailable === 0) {
				this.msg = 'Loads available: 0. Waiting for cooldown.';
				await new Promise(r => setTimeout(r, 5000));
				continue;
			}
			const chunkTabs = Array.from(this.selectedTabs.values()).splice(
				0,
				this.stashLoadsAvailable < 5 ? 1 : LOAD_AT_ONE_ITERATION
			);
			this.msg = `${new Date().toLocaleTimeString('ru')}: Loading ${chunkTabs.length} tabs data`;
			try {
				await Promise.all(
					chunkTabs.map(async ({ id, name }) => {
						if (!this.stashLoader) {
							throw new Error('No stash loader');
						}
						const sample = await this.stashLoader.sampleFromTab(id, league);
						this.emit<Events['sample-from-tab']>('sample-from-tab', { sample, league, name });
						this.selectedTabs.delete(id);
						this.selectedTabs = new Map(this.selectedTabs);
						this.stashLoadsAvailable--;
						setTimeout(() => {
							this.stashLoadsAvailable++;
						}, SECS_300);
					})
				);
			} catch (err) {
				if (typeof err === 'object' && err !== null && 'message' in err) {
					if (typeof err.message === 'string') {
						this.msg = err.message;
					}
				}
				this.fetchingStash = false;

				this.countdown = 0;
				this.selectedTabs = new Map();
				throw err;
			}
			if (this.selectedTabs.size === 0) break;

			// Countdown
			if (this.#countdownTimer) {
				clearInterval(this.#countdownTimer);
				this.#countdownTimer = null;
			}
			this.countdown = SLEEP_SECS;
			this.#countdownTimer = setInterval(() => {
				if (this.countdown <= 0) {
					if (this.#countdownTimer) {
						clearInterval(this.#countdownTimer);
						this.#countdownTimer = null;
					}
				} else {
					this.countdown--;
					this.msg = `Loaded. Now ${this.countdown}s sleep`;
				}
			}, 1000);

			this.msg = `Loaded. Now ${SLEEP_SECS}s sleep`;
			await new Promise(r => setTimeout(r, SLEEP_SECS * 1000));
		}

		this.fetchingStash = false;
		this.msg = '';
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
