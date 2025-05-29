import { LitElement, PropertyValueMap, css, html, nothing } from 'lit';
import { customElement, property, query } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib.js';
import { type League, tradeLeagues, leagues as allLeagues } from '@divicards/shared/types.js';
import '@shoelace-style/shoelace/dist/components/select/select.js';
import '@shoelace-style/shoelace/dist/components/option/option.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import { LeagueChangeEvent } from './events/change/league.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-league-select': LeagueSelectElement;
	}
}

export interface Events {
	'change:league': LeagueChangeEvent;
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

@customElement('e-league-select')
export class LeagueSelectElement extends LitElement {
	@property({ type: Boolean, reflect: true }) trade = false;
	@property({ type: String, reflect: true }) league: League = ACTIVE_LEAGUE;
	@property() privateLeague: string = PrivateLeagueStorage.load() ?? '';
	@property({ type: Boolean, reflect: true, attribute: 'with-private-league-input' }) withPrivateLeagueInput = false;
	@query('sl-select', true)
	select!: HTMLSelectElement;

	protected willUpdate(map: PropertyValueMap<this>): void {
		if (map.has('privateLeague')) {
			PrivateLeagueStorage.save(this.privateLeague);
		}
	}

	get value(): string {
		return this.select.value;
	}

	focus(): void {
		this.select.focus();
	}

	protected override render() {
		const leagues = this.trade ? tradeLeagues : allLeagues;

		const options = html`${leagues.map(
			league => html`<sl-option .value=${SlConverter.toSlValue(league)}>${league}</sl-option>`
		)}`;

		return html`<div class="league-select">
			<sl-select
				.helpText=${`Choose league`}
				size="small"
				.value=${SlConverter.toSlValue(this.league)}
				@sl-change="${this.#emitLeagueChange}"
				id="league"
			>
				${options}
			</sl-select>
			${this.withPrivateLeagueInput
				? html`<sl-input
						class="private-league-input"
						.value=${this.privateLeague}
						@sl-input=${this.#onPrivateLeagueInput}
						id="custom-league-input"
						type="text"
						.helpText=${`Private league`}
						size="small"
				  ></sl-input>`
				: nothing}
		</div>`;
	}

	#onPrivateLeagueInput(e: InputEvent) {
		const target = e.target as HTMLInputElement;
		this.privateLeague = target.value;
		this.league = this.privateLeague;
		this.dispatchEvent(new LeagueChangeEvent(this.league));
	}

	override firstUpdated() {
		this.select.value = this.league;
	}

	async #emitLeagueChange() {
		this.league = SlConverter.fromSlValue<League>(this.select.value);
		await this.updateComplete;
		this.dispatchEvent(new LeagueChangeEvent(this.league));
	}

	static styles = css`
		.league-select {
			display: flex;
			gap: 0.2rem;
		}
		.private-league-input {
			width: 10ch;
		}
	`;
}

class PrivateLeagueStorage {
	static #key = 'CUSTOM_LEAGUE';
	static save(s: string) {
		localStorage.setItem(this.#key, s);
	}

	static load(): string | null {
		return localStorage.getItem(this.#key);
	}
}
