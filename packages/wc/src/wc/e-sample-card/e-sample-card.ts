import { html, css, nothing, LitElement } from 'lit';
import { LeagueSelectElement } from '../e-league-select';
import '../e-league-select';
import { BasePopupElement } from '../e-base-popup';
import './e-fixed-names/e-fixed-names';
import './e-not-cards/e-not-cards';
import { DivinationCardsSample, League, TradeLeague, isTradeLeague } from '@divicards/shared/types';
import { customElement, property, query } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { classMap } from 'lit/directives/class-map.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/range/range.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import SlRange from '@shoelace-style/shoelace/dist/components/range/range.js';
import './e-sample-table/e-sample-table';
import '../e-base-popup';
import { SampleTableElement } from './e-sample-table/e-sample-table';
import { emit } from '../../utils';
import { LeagueChangeEvent } from '../events/change/league';

export interface Props {
	league?: TradeLeague;
	filename: string;
	selected: boolean | null;
	uuid: string;
	minimumCardPrice: number;
	sample: DivinationCardsSample;
}

export interface Events {
	'upd:selected': SampleCardElement['selected'];
	'change:league': LeagueChangeEvent;
	'upd:minimumCardPrice': SampleCardElement['minimumCardPrice'];
	'google-sheets-clicked': { sample: DivinationCardsSample; league: League };
	'save-to-file-clicked': { sample: DivinationCardsSample; league: League; filename: string };
}

export type Events2 = {
	[DeleteThisSampleEvent.tag]: DeleteThisSampleEvent;
};

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

@customElement('e-sample-card')
export class SampleCardElement extends LitElement {
	@property({ reflect: true }) league: TradeLeague = ACTIVE_LEAGUE;
	@property({ reflect: true }) filename: string = 'NO FILE NAMENO FILE NAME';
	@property({ type: Boolean, reflect: true }) selected: boolean | null = false;
	@property({ reflect: true }) uuid: string = 'NO ID';
	@property({ type: Number, reflect: true, attribute: 'minimum-card-price' }) minimumCardPrice: number = 0;
	@property({ type: Object }) sample: DivinationCardsSample = { notCards: [], fixedNames: [], cards: [] };

	@query('e-base-popup#table-popup') tablePopup!: BasePopupElement;
	@query('input#selected-checkbox') selectedCheckbox!: HTMLInputElement;
	@query('e-league-select') leagueSelect!: LeagueSelectElement;
	@query('#minimum-card-price-slider') priceSlider!: HTMLInputElement;
	@query('e-sample-table') table!: SampleTableElement;
	@query('sl-range') rangeEl!: SlRange;

	protected override render() {
		return html`<div
			class=${classMap({
				'sample-card': true,
				'sample-card--selected': Boolean(this.selected),
			})}
		>
			<p class="filename">${this.filename}</p>

			<sl-icon-button
				@click=${this.#emit_delete_this_sample}
				id="btn-delete"
				class="btn-delete"
				name="x-lg"
			></sl-icon-button>
			<div class="minor-icons">
				${this.sample.fixedNames.length > 0
					? html`<e-fixed-names .fixedNames=${this.sample.fixedNames}></e-fixed-names>`
					: nothing}
				${this.sample.notCards.length > 0
					? html`<e-not-cards .notCards=${this.sample.notCards}></e-not-cards>`
					: nothing}
			</div>
			<svg
				class="grid-icon"
				@click=${this.#openSampleTablePopup}
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 512 512"
			>
				<path
					fill="currentColor"
					d="M47.547 63.547v384.906a16 16 0 0 0 16 16h384.906a16 16 0 0 0 16-16V63.547a16 16 0 0 0-16-16H63.547a16 16 0 0 0-16 16Zm288.6 16h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm-128.3-256.6h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm-128.3-256.6h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Z"
				/>
			</svg>
			<sl-range
				id="minimum-card-price-slider"
				class="slider"
				name=""
				id=""
				min="0"
				max="500"
				.value=${this.minimumCardPrice}
				@sl-input=${this.#onMinPriceRange}
			></sl-range>
			<div class="total-price">
				<p>${format(this.filteredSummary.value)}</p>
				<img width="35" height="35" class="chaos-img" src="/chaos.png" alt="chaos" />
			</div>
			<div class="cards-amount">
				<p>${this.filteredSummary.amount}</p>
				<img width="35" height="35" src="/divination-card.png" alt="Divination card" />
			</div>
			<e-league-select
				trade
				.league=${this.league}
				@change:league=${this.#handle_league_change}
			></e-league-select>
			<div class="export-buttons">
				<sl-button size="large" @click=${this.#onSaveToFileClicked}>
					<sl-icon style="font-size:1.6rem" name="filetype-csv"></sl-icon>
					Save to file</sl-button
				>
				<sl-button @click=${this.#emitGoogleSheetsClicked} size="large">
					<sl-icon style="font-size:1.6rem" name="file-earmark-spreadsheet"></sl-icon>
					Export to Google Sheets</sl-button
				>
			</div>
			${this.selected === null
				? nothing
				: html`<input
						class="checkbox"
						type="checkbox"
						.checked=${this.selected}
						id="selected-checkbox"
						@change=${this.#onSelectedClicked}
					/>`}
			<e-base-popup id="table-popup">
				<e-sample-table .cards=${this.sample.cards}> </e-sample-table>
			</e-base-popup>
		</div>`;
	}

	get filteredCards() {
		return this.sample.cards.filter(card => {
			return (card.price ?? 0) >= this.minimumCardPrice;
		});
	}

	get filteredSummary() {
		let value = 0;
		let amount = 0;

		for (const card of this.filteredCards) {
			value += card.sum ?? 0;
			amount += card.amount;
		}

		return {
			amount,
			value,
		};
	}

	#onSaveToFileClicked() {
		emit<Events['save-to-file-clicked']>(this, 'save-to-file-clicked', {
			sample: this.sample,
			league: this.league,
			filename: this.filename,
		});
	}

	#emitGoogleSheetsClicked(): void {
		emit<Events['google-sheets-clicked']>(this, 'google-sheets-clicked', {
			sample: this.sample,
			league: this.league,
		});
	}

	#openSampleTablePopup(): void {
		this.tablePopup.showModal();
	}

	#onSelectedClicked() {
		if (this.selected === null) return;
		this.selected = this.selectedCheckbox.checked;
		emit<Events['upd:selected']>(this, 'upd:selected', this.selected);
	}

	#handle_league_change(e: LeagueChangeEvent): void {
		if (!isTradeLeague(e.league)) {
			return;
		}
		this.league = e.league;
		this.dispatchEvent(new LeagueChangeEvent(e.league));
	}

	#emit_delete_this_sample(): void {
		this.dispatchEvent(new DeleteThisSampleEvent(this.uuid));
	}

	#onMinPriceRange(e: Event): void {
		if (!(e.target && 'value' in e.target)) {
			return;
		}

		this.minimumCardPrice = Number(e.target.value);
		emit<Events['upd:minimumCardPrice']>(this, 'upd:minimumCardPrice', this.minimumCardPrice);
	}

	static override styles = [
		css`
			:host {
				--border-color: rgba(255, 255, 255, 0.3);
				--border-radius: 0.25rem;
			}

			.sample-card {
				position: relative;
				padding-inline: 1rem;
				padding-top: 1.4rem;
				padding-bottom: 0.4rem;
				display: flex;
				flex-direction: column;
				align-items: center;
				justify-content: space-between;
				gap: 1rem;
				width: fit-content;
				box-shadow:
					rgba(0, 0, 0, 0.02) 0px 1px 3px 0px,
					rgba(27, 31, 35, 0.15) 0px 0px 0px 1px;

				/* max-height: 320px; */
				width: 250px;
				height: 530px;

				border: 1px solid black;
				border-color: var(--border-color);
				border-radius: var(--border-radius);
				background-color: var(--sl-color-gray-100);
				transition: 0.2s border-color;
			}

			.sample-card--selected {
				border-color: var(--sl-color-green-600);
			}

			.icon {
				cursor: pointer;
			}

			p {
				margin: 0;
			}

			.minor-icons {
				position: absolute;
				top: 30%;
				left: 20px;
			}

			.file-error {
				border-color: red;
			}

			.filename {
				font-size: 1rem;
				letter-spacing: -0.4px;
				overflow: hidden;
				max-height: 60px;
				max-width: 100%;
				margin-top: 1.2rem;
			}

			.filename:hover {
				overflow: visible;
			}

			.slider-box {
				display: flex;
				justify-content: center;
				align-items: center;
				gap: 0.5rem;
			}

			.btn-delete {
				position: absolute;
				top: 0;
				right: 0;
				padding: 0.2rem;
				border: none;
				background-color: transparent;
				cursor: pointer;
			}

			sl-icon {
				color: var(--sl-color-green-600);
			}

			.export-buttons {
				margin-top: 2rem;
				display: flex;
				flex-direction: column;
			}

			.checkbox {
				background-color: red;
				transform: scale(2);
				accent-color: var(--sl-color-green-600);
				cursor: pointer;

				position: absolute;
				bottom: 0;
				right: 0;
				width: 10px;
				height: 10px;
				transform: translate(50%, 50%) scale(2);
			}

			.export-to-google-docs {
				margin-top: auto;
				cursor: pointer;
			}

			.total-price,
			.cards-amount {
				display: flex;
				justify-content: center;
				align-items: center;
				font-size: 2rem;
			}

			.grid-icon {
				display: block;
				cursor: pointer;
				padding: 0;
				margin: 0;
				width: 100px;
				height: 100px;
			}
		`,
	];
}

declare global {
	interface HTMLElementTagNameMap {
		'e-sample-card': SampleCardElement;
	}
}

export class DeleteThisSampleEvent extends Event {
	static readonly tag = 'sample__delete';
	uuid: string;
	constructor(uuid: string, options?: EventInit) {
		super(DeleteThisSampleEvent.tag, options);
		this.uuid = uuid;
	}
}
