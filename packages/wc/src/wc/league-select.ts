import { BaseElement } from './base-element';
import { PropertyValueMap, css, html } from 'lit';
import { property, query } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { League, tradeLeagues, leagues as allLeagues } from '@divicards/shared/types';
import '@shoelace-style/shoelace/dist/components/select/select.js';
import '@shoelace-style/shoelace/dist/components/option/option.js';

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

export class SlConverter {
	static #SL_DELIMETER = 'sl-v' as const;
	static toSlValue<T extends string>(s: T): string {
		return s.replaceAll(' ', this.#SL_DELIMETER);
	}
	static fromSlValue<T extends string>(s: string): T {
		return s.replaceAll(this.#SL_DELIMETER, ' ') as T;
	}
}

export class LeagueSelectElement extends BaseElement {
	static override tag = 'wc-league-select';
	static override styles = [this.baseStyles, styles];

	@property({ type: Boolean, reflect: true }) trade = false;
	@property({ type: String, reflect: true }) league: League = ACTIVE_LEAGUE;
	@query('sl-select', true) select!: HTMLSelectElement;

	get value() {
		return this.select.value;
	}

	focus() {
		this.select.focus();
	}

	protected override render() {
		const leagues = this.trade ? tradeLeagues : allLeagues;

		const options = html`${leagues.map(
			league => html`<sl-option .value=${SlConverter.toSlValue(league)}>${league}</sl-option>`
		)}`;

		return html`<div class="league-select">
			<sl-select .value=${SlConverter.toSlValue(this.league)} @sl-change="${this.#emitLeagueChange}" id="league">
				${options}
			</sl-select>
		</div>`;
	}

	override firstUpdated() {
		this.select.value = this.league;
	}

	async #emitLeagueChange() {
		this.league = SlConverter.fromSlValue<League>(this.select.value);
		await this.updateComplete;
		this.emit<Events['upd:league']>('upd:league', this.league);
	}
}
