import { stashes as loadStashes } from './../../../../app/src/poe/api';
import { html, css, PropertyValueMap, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import { HelpTipElement } from '../help-tip';
import { TabBadgeElement } from './tab-badge';
import { LeagueSelectElement } from '../league-select';
import { property, state } from 'lit/decorators.js';
import { StashTab } from '@divicards/shared/poe.types';
import { League } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { TabBadgeGroupElement } from './tab-badge-group';
import { classMap } from 'lit/directives/class-map.js';
import { command } from '../../../../app/src/command';

declare global {
	interface HTMLElementTagNameMap {
		'wc-stashes-view': StashesViewElement;
	}
}

const styles = css`
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

	.msg {
		font-size: 2rem;
		max-width: max-content;
		margin-inline: auto;
		margin-top: 1rem;
		visibility: hidden;
		min-height: 2rem;
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

export class Events {
	'close': void;
	// ---
	/** from tab-badge-group */
	'upd:selectedTabs': Set<TabBadgeElement['tabId']>;
	/** from tab-badge-group */
	'upd:nameQuery': string;
	/** from tab-badge-group */
	'upd:PerPage': number;
	/** from tab-badge-group */
	'upd:page': number;

	'tab-data': { tab: StashTab; league: League };

	// ---
	/**  event from TabBadgeElement */
	'tab-select': { tabId: TabBadgeElement['tabId']; selected: boolean };
}

export class StashesViewElement extends BaseElement {
	static define(tag = this.htmlTag) {
		if (!customElements.get(tag)) {
			customElements.define(tag, StashesViewElement);
			HelpTipElement.define();
			LeagueSelectElement.define();
			TabBadgeElement.define();
			TabBadgeGroupElement.define();
		}
	}
	static htmlTag = 'wc-stashes-view';
	static styles = [this.baseStyles, styles];

	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;

	#countdownTimer: ReturnType<typeof setInterval> | null = null;
	@state() selectedTabs: Set<string> = new Set();
	@state() stashes: StashTab[] = [];
	@state() noStashesMessage: string = '';
	@state() msg: string = '';
	@state() fetchingStash: boolean = false;
	@state() countdown = 0;

	protected willUpdate(map: PropertyValues<this>): void {
		if (map.has('league')) {
			this.stashes = [];
		}
	}

	async #onLoadItemsClicked() {
		console.log('#onGetData: TODO');
		await this.fetchStashesContents(Array.from(this.selectedTabs), this.league);
		this.selectedTabs = new Set();
		console.log(this.selectedTabs);
	}

	#onCloseClicked() {
		this.emit('close');
	}

	async #onLoadStashesList() {
		// onStashes(league)
		this.noStashesMessage = '';
		this.stashes = await loadStashes(this.league);
		console.log(this.stashes);
		if (!this.stashes.length) {
			this.noStashesMessage = 'No stashes here. Try to change the league';
		}
	}

	#onLeagueSelected(e: CustomEvent<League>) {
		this.league = e.detail;
	}

	#onUpdSelectedTabs(e: CustomEvent<Events['upd:selectedTabs']>) {
		const set = (e as CustomEvent<Events['upd:selectedTabs']>).detail;
		this.selectedTabs = new Set(set);
		console.log(this.selectedTabs);
	}

	render() {
		console.log(this.selectedTabs.size);
		return html`<div class="main-stashes-component">
			<div class="controls">
				<div class="league-stashes">
					<wc-league-select :league="league" @upd:league=${this.#onLeagueSelected}></wc-league-select>
					<button @click=${this.#onLoadStashesList}>Stashes</button>
					<wc-help-tip>
						<p>Select tabs by clicking on them. Then click LOAD ITEMS button</p>
					</wc-help-tip>
				</div>

				<button
					class=${classMap({ 'not-visible': this.selectedTabs.size === 0, 'btn-load-items': true })}
					.disabled=${this.selectedTabs.size === 0 || this.fetchingStash}
					@click=${this.#onLoadItemsClicked}
				>
					load items
				</button>

				<button @click=${this.#onCloseClicked} class="btn-close">Close</button>
			</div>

			<p class=${classMap({ visible: this.fetchingStash, msg: true })}>${this.msg}</p>
			<p class=${classMap({ visible: this.noStashesMessage.length > 0, msg: true })}>${this.noStashesMessage}</p>

			<wc-tab-badge-group
				league=${this.league}
				.stashes=${this.stashes}
				.selectedTabs=${this.selectedTabs}
				@upd:selectedTabs=${this.#onUpdSelectedTabs}
			></wc-tab-badge-group>
		</div>`;
	}

	async fetchStashesContents(ids: string[], league: League) {
		// const tradeLeague = isTradeLeague(league) ? league : ACTIVE_LEAGUE;
		const SLEEP_SECS = 10;
		const LOAD_AT_ONE_ITERATION = 5;
		const stashIds = ids.slice();
		const result: StashTab[] = [];
		this.fetchingStash = true;
		while (stashIds.length > 0) {
			const chunkIds = stashIds.splice(0, LOAD_AT_ONE_ITERATION);
			this.msg = `${new Date().toLocaleTimeString('ru')}: Loading ${chunkIds.length} tabs data`;
			const r = await Promise.all(
				chunkIds.map(async stashId => {
					const { stash: tab } = await command('stash', { stashId, league });
					this.emit<Events['tab-data']>('tab-data', { tab, league });
					this.selectedTabs.delete(tab.id);
					this.selectedTabs = new Set(this.selectedTabs);

					return tab;
				})
			);
			result.push(...r);
			if (stashIds.length === 0) break;

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

		return result;
	}
}
