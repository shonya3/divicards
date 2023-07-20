import { html, css, nothing } from 'lit';
import { BaseElement } from '../base-element';
import { LeagueSelectElement } from '../league-select';
import { DivTableElement } from '../div-table/div-table';
import { BasePopupElement } from '../base-popup';
import { FixedNamesElement } from './fixed-names/fixed-names';
import { NotCardsElement } from './not-cards/not-cards';
import { DivinationCardsSample, League, TradeLeague, isTradeLeague } from '@divicards/shared/types';
import { property, query } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';

declare global {
	interface HTMLElementTagNameMap {
		'wc-file-card': FileCardElement;
	}
}

export interface FileCardProps {
	league: League;
	filename: string;
	href: string;
	selected: boolean | null;
	uuid: string;
	valid: boolean;
	error: string | null;
	minimumCardPrice: number;
	sample: DivinationCardsSample;
	isReady: boolean;
}

const styles = css`
	.league {
		display: flex;
		gap: 0.4rem;
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
		min-height: 400px;

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
	}
`;

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

export interface Events {
	'update#selected': FileCardElement['selected'];
	'update#league': FileCardElement['league'];
	'update#minimumCardPrice': FileCardElement['minimumCardPrice'];
	delete: FileCardElement['uuid'];
}

export class FileCardElement extends BaseElement {
	static define(tag = 'wc-file-card') {
		if (!customElements.get(tag)) {
			customElements.define(tag, FileCardElement);
			LeagueSelectElement.define();
			DivTableElement.define();
			BasePopupElement.define();
			FixedNamesElement.define();
			NotCardsElement.define();
		}
	}
	static htmlTag = 'wc-file-card';
	static styles = [this.baseStyles, styles];

	@property({ reflect: true }) league: TradeLeague = ACTIVE_LEAGUE;
	@property({ reflect: true }) filename: string = 'NO FILE NAME';
	@property({ reflect: true }) href: string = 'NO HREF';
	@property({ type: Boolean, reflect: true }) selected: boolean | null = false;
	@property({ reflect: true }) uuid: string = 'NO ID';
	@property({ reflect: true, type: Boolean }) valid: boolean = true;
	@property({ reflect: true }) error: string | null = null;
	@property({ type: Number, reflect: true, attribute: 'minimum-card-price' }) minimumCardPrice: number = 0;
	@property({ type: Object }) sample!: DivinationCardsSample;
	@property({ type: Boolean, attribute: 'is-ready', reflect: true }) isReady: boolean = false;

	@query('wc-base-popup#table-popup') tablePopup!: BasePopupElement;
	@query('input#selected-checkbox') selectedCheckbox!: HTMLInputElement;
	@query('wc-league-select') leagueSelect!: LeagueSelectElement;
	@query('#minimum-card-price-slider') priceSlider!: HTMLInputElement;

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

	#onGridIconClicked() {
		this.tablePopup.open();
	}

	#onSelectedClicked() {
		if (this.selected === null) return;
		this.selected = this.selectedCheckbox.checked;
		this.emit<Events['update#selected']>('update#selected', this.selected);
		this.emit<Events['update#selected']>('update-selected', this.selected);
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
			this.emit<Events['update#minimumCardPrice']>('update#minimumCardPrice', this.minimumCardPrice);
		}
	}

	render() {
		return html`<div class="file" :class="{ 'file-error': error, 'file-selected': selected }">
			<p class="filename" :class="{ 'filename--error': error }">${this.filename}</p>
			<button @click=${this.#onBtnDeleteClicked} id="btn-delete" class="btn-delete">X</button>

			${this.isReady
				? html`${this.error
						? html`<p>${this.error}</p>`
						: html`<div class="minor-icons">
									${this.sample.fixedNames.length > 0
										? html`<wc-fixed-names .fixedNames=${this.sample.fixedNames}></wc-fixed-names>`
										: nothing}
									${this.sample.notCards.length > 0
										? html`<wc-not-cards .notCards=${this.sample.notCards}></wc-not-cards>`
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
									@update#league=${this.#onLeagueSelected}
								></wc-league-select>

								<a class="download" .download=${this.filename} .href=${this.href}>Download</a>

								${this.selected === null
									? nothing
									: html`<input
											class="checkbox"
											v-if="valid && selected != null"
											type="checkbox"
											:checked="selected"
											id="selected-checkbox"
											@change=${this.#onSelectedClicked}
									  />`}

								<wc-base-popup id="table-popup">
									${this.error
										? html`<p>${this.error}</p>`
										: html`<wc-div-table .cards=${this.sample.cards}></wc-div-table>`}
								</wc-base-popup>`}`
				: nothing}
		</div>`;
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
}
