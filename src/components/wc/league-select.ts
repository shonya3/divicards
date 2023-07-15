import { BaseElement } from './base-element';
import { css, html } from 'lit';
import { property, query } from 'lit-element/decorators.js';
import { leagues as allLeagues, tradeLeagues, League } from '../../types';
import { ACTIVE_LEAGUE } from '../../lib';

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

export class LeagueSelectElement extends BaseElement {
	static override htmlTag: string = 'wc-league-select';
	static override styles = styles;

	@property({ type: Boolean, reflect: true }) trade = false;
	@property({ type: String, reflect: true }) league: League = ACTIVE_LEAGUE;
	@query('select', true) select!: HTMLSelectElement;

	get value() {
		return this.select.value;
	}

	override render() {
		const leagues = this.trade ? tradeLeagues : allLeagues;

		const options = html`${leagues.map(league => html`<option .value=${league}>${league}</option>`)}`;

		return html`<div class="league-select">
			<label for="league">League</label>
			<select .value=${this.league} @change="${this.#emitLeagueChange}" id="league">
				${options}
			</select>
		</div>`;
	}

	#emitLeagueChange() {
		this.emit('league-change', this.select.value);
	}
}
