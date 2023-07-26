import { html, css, nothing } from 'lit';
import { BaseElement } from '../base-element';
import { LeagueSelectElement } from '../league-select';
import { DivTableElement } from '../div-table/div-table';
import { BasePopupElement } from '../base-popup';
import { FixedNamesElement } from './fixed-names/fixed-names';
import { NotCardsElement } from './not-cards/not-cards';
import { DivinationCardsSample, League, Result, TradeLeague, isTradeLeague } from '@divicards/shared/types';
import { property, query } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { classMap } from 'lit/directives/class-map.js';
const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

declare global {
	interface HTMLElementTagNameMap {
		'wc-file-card': FileCardElement;
	}
}

export interface FileCardProps {
	league: League;
	filename: string;
	selected: boolean | null;
	uuid: string;
	minimumCardPrice: number;
	sample: Result<DivinationCardsSample>;
}

export interface Events {
	'upd:selected': FileCardElement['selected'];
	'upd:league': FileCardElement['league'];
	'upd:minimumCardPrice': FileCardElement['minimumCardPrice'];
	delete: FileCardElement['uuid'];
}

export class FileCardElement extends BaseElement {
	static override defineList = [
		LeagueSelectElement,
		DivTableElement,
		BasePopupElement,
		FixedNamesElement,
		NotCardsElement,
	];
	static override tag = 'wc-file-card';

	@property({ reflect: true }) league: TradeLeague = ACTIVE_LEAGUE;
	@property({ reflect: true }) filename: string = 'NO FILE NAME';
	@property({ type: Boolean, reflect: true }) selected: boolean | null = false;
	@property({ reflect: true }) uuid: string = 'NO ID';
	@property({ type: Number, reflect: true, attribute: 'minimum-card-price' }) minimumCardPrice: number = 0;
	@property({ type: Object }) sample: Result<DivinationCardsSample> = { type: 'err', error: 'No sample data' };

	@query('wc-base-popup#table-popup') tablePopup!: BasePopupElement;
	@query('input#selected-checkbox') selectedCheckbox!: HTMLInputElement;
	@query('wc-league-select') leagueSelect!: LeagueSelectElement;
	@query('#minimum-card-price-slider') priceSlider!: HTMLInputElement;

	get filteredCards() {
		if (this.sample.type === 'ok') {
			return this.sample.data.cards.filter(card => {
				return (card.price ?? 0) >= this.minimumCardPrice;
			});
		} else return [];
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

	#onGridIconClicked() {
		this.tablePopup.open();
	}

	#onSelectedClicked() {
		if (this.selected === null) return;
		this.selected = this.selectedCheckbox.checked;
		this.emit<Events['upd:selected']>('upd:selected', this.selected);
	}

	#onLeagueSelected(e: CustomEvent<League>) {
		const league = e.detail;
		if (!isTradeLeague(league)) {
			e.stopPropagation();
			return;
		}
		this.league = league;
	}

	#onBtnDeleteClicked() {
		this.emit<Events['delete']>('delete', this.uuid);
	}

	#onMinPriceSlider(e: InputEvent) {
		const target = e.composedPath()[0];
		if (target instanceof HTMLInputElement && target.matches('#minimum-card-price-slider')) {
			this.minimumCardPrice = Number(target.value);
			this.emit<Events['upd:minimumCardPrice']>('upd:minimumCardPrice', this.minimumCardPrice);
		}
	}

	protected override render() {
		return html`<div
			class=${classMap({
				file: true,
				'file-error': this.sample.type === 'err',
				'file-selected': Boolean(this.selected),
			})}
		>
			<p class=${classMap({ filename: true, 'filename--error': this.sample.type === 'err' })}>${this.filename}</p>
			<button @click=${this.#onBtnDeleteClicked} id="btn-delete" class="btn-delete">X</button>
			${this.chunk()}
		</div>`;
	}

	get urlObject() {
		if (this.sample.type === 'err') throw new Error('Cannot download erroneus file');
		return URL.createObjectURL(new File([this.sample.data.csv ?? ''], this.filename));
	}

	protected chunk() {
		return html`${this.sample.type === 'err'
			? html`<p>${this.sample.error}</p>`
			: html`<div class="minor-icons">
						${this.sample.data.fixedNames.length > 0
							? html`<wc-fixed-names .fixedNames=${this.sample.data.fixedNames}></wc-fixed-names>`
							: nothing}
						${this.sample.data.notCards.length > 0
							? html`<wc-not-cards .notCards=${this.sample.data.notCards}></wc-not-cards>`
							: nothing}
					</div>
					${this.gridIcon()}
					<label class="slider-box">
						<span>${this.minimumCardPrice}</span>
						<input
							id="minimum-card-price-slider"
							class="slider"
							type="range"
							name=""
							id=""
							min="0"
							max="500"
							.value=${this.minimumCardPrice}
							@input=${this.#onMinPriceSlider}
						/>
					</label>

					<div class="total-price">
						<p>${format(this.filteredSummary.value)}</p>
						<img width="35" height="35" class="chaos-img" src="/chaos.png" alt="chaos" />
					</div>
					<div class="cards-amount">
						<p>${this.filteredSummary.amount}</p>
						<img width="35" height="35" src="/divination-card.png" alt="Divination card" />
					</div>

					<wc-league-select
						trade
						.league=${this.league}
						@upd:league=${this.#onLeagueSelected}
					></wc-league-select>

					<a class="download" .download=${this.filename} .href=${this.urlObject}>Download</a>

					${this.selected === null
						? nothing
						: html`<input
								class="checkbox"
								type="checkbox"
								.checked=${this.selected}
								id="selected-checkbox"
								@change=${this.#onSelectedClicked}
						  />`}

					<wc-base-popup id="table-popup">
						<wc-div-table .cards=${this.sample.data.cards}></wc-div-table>
					</wc-base-popup>`}`;
	}

	protected gridIcon() {
		return html`<svg
			class="grid-icon"
			@click=${this.#onGridIconClicked}
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 512 512"
		>
			<path
				fill="currentColor"
				d="M47.547 63.547v384.906a16 16 0 0 0 16 16h384.906a16 16 0 0 0 16-16V63.547a16 16 0 0 0-16-16H63.547a16 16 0 0 0-16 16Zm288.6 16h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm-128.3-256.6h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm-128.3-256.6h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Z"
			/>
		</svg>`;
	}

	static override styles = [
		this.baseStyles,
		css`
			.league {
				display: flex;
				gap: 0.4rem;
			}

			p {
				margin: 0;
			}

			.minor-icons {
				position: absolute;
				top: 30%;
				left: 20px;
			}
			.file {
				position: relative;
				padding: 1rem;
				padding-block: 1.4rem;
				display: flex;
				flex-direction: column;
				align-items: center;
				justify-content: center;
				gap: 1rem;
				width: fit-content;
				box-shadow: rgba(0, 0, 0, 0.02) 0px 1px 3px 0px, rgba(27, 31, 35, 0.15) 0px 0px 0px 1px;

				width: 250px;
				/* max-height: 320px; */
				height: 450px;

				border: 2px solid black;
				border-color: var(--border-color);
				transition: 0.2s border-color;
			}

			.icon {
				cursor: pointer;
			}

			.file-error {
				border-color: red;
			}

			.file-selected {
				border-color: green;
			}

			.filename {
				font-size: 1rem;
				letter-spacing: -0.4px;
				overflow: hidden;
				max-height: 60px;
				max-width: 100%;
			}

			.filename:hover {
				overflow: visible;
				/* position: absolute; */
			}

			.filename--error {
				color: red;
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
				/* transform: translate(-50%, 50%); */
				padding: 0.2rem;
				border: none;
				background-color: transparent;
				cursor: pointer;
			}

			.checkbox {
				background-color: red;
				padding: 1rem;
				transform: scale(2);
				accent-color: green;
				cursor: pointer;

				position: absolute;
				bottom: 0;
				right: 0;
				transform: translate(15%, 15%) scale(2);
			}

			.download {
				/* position: absolute; */
				bottom: 0;
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
				width: 96px;
				height: 96px;
				padding: 0;
				margin: 0;
			}
		`,
	];
}
