import { BaseElement } from './base-element';
import { PropertyValueMap, css, html } from 'lit';
import { property, query } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { League, tradeLeagues, leagues as allLeagues } from '@divicards/shared/types';

declare global {
	interface HTMLElementTagNameMap {
		'wc-league-select': LeagueSelectElement;
	}
}

const styles = css`
	.league-select {
		display: flex;
		gap: 0.2rem;
	}
`;

export interface Events {
	'upd:league': League;
}

export class LeagueSelectElement extends BaseElement {
	static override htmlTag: string = 'wc-league-select';
	static override styles = styles;

	@property({ type: Boolean, reflect: true }) trade = false;
	@property({ type: String, reflect: true }) league: League = ACTIVE_LEAGUE;
	@query('select', true) select!: HTMLSelectElement;

	get value() {
		return this.select.value;
	}

	focus() {
		this.select.focus();
	}

	render() {
		const leagues = this.trade ? tradeLeagues : allLeagues;

		const options = html`${leagues.map(league => html`<option .value=${league}>${league}</option>`)}`;

		return html`<div class="league-select">
			<label for="league">League</label>
			<select .value=${this.league} @change="${this.#emitLeagueChange}" id="league">
				${options}
			</select>
		</div>`;
	}

	firstUpdated() {
		this.select.value = this.league;
	}

	async #emitLeagueChange() {
		this.league = this.select.value as League;
		await this.updateComplete;
		this.emit<Events['upd:league']>('upd:league', this.league);
	}
}
