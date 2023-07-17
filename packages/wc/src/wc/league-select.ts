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

	protected firstUpdated(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
		this.select.value = this.league;
	}

	#emitLeagueChange() {
		this.league = this.select.value as League;
		this.emit('league-change', this.select.value);
	}
}
