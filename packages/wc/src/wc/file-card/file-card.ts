import { html, css, nothing } from 'lit';
import { BaseElement } from '../base-element';
import { LeagueSelectElement } from '../league-select';
import { DivTableElement } from '../div-table/div-table';
import { BasePopupElement } from '../base-popup';
import { FixedNamesElement } from './fixed-names/fixed-names';
import { NotCardsElement } from './not-cards/not-cards';
import { IconButtonElement } from '../icon-button/icon-button';
import { DivinationCardsSample, League, TradeLeague, isTradeLeague } from '@divicards/shared/types';
import { property, query } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { classMap } from 'lit/directives/class-map.js';
import { ToGoogleSheetsElement } from '../to-google-sheets/to-google-sheets';
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
	sample: DivinationCardsSample;
}

export interface Events {
	'upd:selected': FileCardElement['selected'];
	'upd:league': FileCardElement['league'];
	'upd:minimumCardPrice': FileCardElement['minimumCardPrice'];
	delete: FileCardElement['uuid'];
	'google-sheets-clicked': { sample: DivinationCardsSample; league: League };
}

export class FileCardElement extends BaseElement {
	static override get defineList() {
		return [
			ToGoogleSheetsElement,
			LeagueSelectElement,
			DivTableElement,
			BasePopupElement,
			FixedNamesElement,
			NotCardsElement,
			IconButtonElement,
		];
	}
	static override tag = 'wc-file-card';

	@property({ reflect: true }) league: TradeLeague = ACTIVE_LEAGUE;
	@property({ reflect: true }) filename: string = 'NO FILE NAME';
	@property({ type: Boolean, reflect: true }) selected: boolean | null = false;
	@property({ reflect: true }) uuid: string = 'NO ID';
	@property({ type: Number, reflect: true, attribute: 'minimum-card-price' }) minimumCardPrice: number = 0;
	@property({ type: Object }) sample: DivinationCardsSample = { csv: '', notCards: [], fixedNames: [], cards: [] };

	@query('wc-base-popup#table-popup') tablePopup!: BasePopupElement;
	@query('input#selected-checkbox') selectedCheckbox!: HTMLInputElement;
	@query('wc-league-select') leagueSelect!: LeagueSelectElement;
	@query('#minimum-card-price-slider') priceSlider!: HTMLInputElement;
	@query('wc-div-table') table!: DivTableElement;

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

	#onSheetsIconClicked() {
		this.emit<Events['google-sheets-clicked']>('google-sheets-clicked', {
			sample: this.sample,
			league: this.league,
		});
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
				'file-selected': Boolean(this.selected),
			})}
		>
			<p class="filename">${this.filename}</p>

			<wc-icon-button @click=${this.#onBtnDeleteClicked} id="btn-delete" class="btn-delete " name="close"
				>Here</wc-icon-button
			>
			${this.chunk()}
		</div>`;
	}

	get urlObject() {
		return URL.createObjectURL(new File([this.sample.csv ?? ''], this.filename));
	}

	protected chunk() {
		return html`<div class="minor-icons">
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
					.value=${String(this.minimumCardPrice)}
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

			<wc-league-select trade .league=${this.league} @upd:league=${this.#onLeagueSelected}></wc-league-select>

			<a class="download" .download=${this.filename} .href=${this.urlObject}>Download</a>
			<div class="export-to-google-docs" @click=${this.#onSheetsIconClicked}>
				<p>Export to Sheets</p>
				<svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 100 100">
					<path
						fill="#70b570"
						d="M59.5,12H27c-2.761,0-5,2.239-5,5v66c0,2.761,2.239,5,5,5h46c2.761,0,5-2.239,5-5V30.5L59.5,12z"
					></path>
					<path fill="#8cc78c" d="M59.5,11.5V25c0,3.038,2.462,5.5,5.5,5.5h13.5L59.5,11.5z"></path>
					<path
						fill="#1f212b"
						d="M73,89H27c-3.309,0-6-2.691-6-6V17c0-3.309,2.691-6,6-6h32.5c0.266,0,0.52,0.105,0.707,0.293 l18.5,18.5C78.895,29.98,79,30.235,79,30.5V83C79,86.309,76.309,89,73,89z M27,13c-2.206,0-4,1.794-4,4v66c0,2.206,1.794,4,4,4h46 c2.206,0,4-1.794,4-4V30.914L59.086,13H27z"
					></path>
					<path
						fill="#1f212b"
						d="M78.5,31H65c-3.309,0-6-2.691-6-6V11.5h1V25c0,2.757,2.243,5,5,5h13.5V31z"
					></path>
					<path
						fill="#1f212b"
						d="M26.5,41c-0.276,0-0.5-0.224-0.5-0.5v-22c0-1.378,1.121-2.5,2.5-2.5h15c0.276,0,0.5,0.224,0.5,0.5 S43.776,17,43.5,17h-15c-0.827,0-1.5,0.673-1.5,1.5v22C27,40.776,26.776,41,26.5,41z"
					></path>
					<path
						fill="#1f212b"
						d="M71.5,84h-11c-0.276,0-0.5-0.224-0.5-0.5s0.224-0.5,0.5-0.5h11c0.827,0,1.5-0.673,1.5-1.5v-6 c0-0.276,0.224-0.5,0.5-0.5s0.5,0.224,0.5,0.5v6C74,82.878,72.879,84,71.5,84z"
					></path>
					<path
						fill="#1f212b"
						d="M73.5,73c-0.276,0-0.5-0.224-0.5-0.5v-32c0-0.276,0.224-0.5,0.5-0.5s0.5,0.224,0.5,0.5v32 C74,72.776,73.776,73,73.5,73z"
					></path>
					<path
						fill="#fefdef"
						d="M66.5,46.5h-33v28h33V46.5z M37.5,50.5H48v4H37.5V50.5z M52,50.5h10.5v4H52V50.5z M37.5,58.5H48v4 H37.5V58.5z M52,58.5h10.5v4H52V58.5z M37.5,66.5H48v4H37.5V66.5z M52,66.5h10.5v4H52V66.5z"
					></path>
					<path
						fill="#1f212b"
						d="M66.5,75h-33c-0.276,0-0.5-0.224-0.5-0.5v-28c0-0.276,0.224-0.5,0.5-0.5h33 c0.276,0,0.5,0.224,0.5,0.5v28C67,74.776,66.776,75,66.5,75z M34,74h32V47H34V74z"
					></path>
					<path
						fill="#1f212b"
						d="M48,55H37.5c-0.276,0-0.5-0.224-0.5-0.5v-4c0-0.276,0.224-0.5,0.5-0.5H48c0.276,0,0.5,0.224,0.5,0.5 v4C48.5,54.776,48.276,55,48,55z M38,54h9.5v-3H38V54z"
					></path>
					<path
						fill="#1f212b"
						d="M62.5,55H52c-0.276,0-0.5-0.224-0.5-0.5v-4c0-0.276,0.224-0.5,0.5-0.5h10.5 c0.276,0,0.5,0.224,0.5,0.5v4C63,54.776,62.776,55,62.5,55z M52.5,54H62v-3h-9.5V54z"
					></path>
					<path
						fill="#1f212b"
						d="M48,63H37.5c-0.276,0-0.5-0.224-0.5-0.5v-4c0-0.276,0.224-0.5,0.5-0.5H48c0.276,0,0.5,0.224,0.5,0.5 v4C48.5,62.776,48.276,63,48,63z M38,62h9.5v-3H38V62z"
					></path>
					<path
						fill="#1f212b"
						d="M62.5,63H52c-0.276,0-0.5-0.224-0.5-0.5v-4c0-0.276,0.224-0.5,0.5-0.5h10.5 c0.276,0,0.5,0.224,0.5,0.5v4C63,62.776,62.776,63,62.5,63z M52.5,62H62v-3h-9.5V62z"
					></path>
					<path
						fill="#1f212b"
						d="M48,71H37.5c-0.276,0-0.5-0.224-0.5-0.5v-4c0-0.276,0.224-0.5,0.5-0.5H48c0.276,0,0.5,0.224,0.5,0.5 v4C48.5,70.776,48.276,71,48,71z M38,70h9.5v-3H38V70z"
					></path>
					<path
						fill="#1f212b"
						d="M62.5,71H52c-0.276,0-0.5-0.224-0.5-0.5v-4c0-0.276,0.224-0.5,0.5-0.5h10.5 c0.276,0,0.5,0.224,0.5,0.5v4C63,70.776,62.776,71,62.5,71z M52.5,70H62v-3h-9.5V70z"
					></path>
					<path
						fill="#1f212b"
						d="M57.5,84h-5c-0.276,0-0.5-0.224-0.5-0.5s0.224-0.5,0.5-0.5h5c0.276,0,0.5,0.224,0.5,0.5 S57.776,84,57.5,84z"
					></path>
					<path
						fill="#1f212b"
						d="M26.5,55c-0.276,0-0.5-0.224-0.5-0.5v-11c0-0.276,0.224-0.5,0.5-0.5s0.5,0.224,0.5,0.5v11 C27,54.776,26.776,55,26.5,55z"
					></path>
					<path
						fill="#1f212b"
						d="M26.5,61c-0.276,0-0.5-0.224-0.5-0.5v-3c0-0.276,0.224-0.5,0.5-0.5s0.5,0.224,0.5,0.5v3 C27,60.776,26.776,61,26.5,61z"
					></path>
				</svg>
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

			<wc-base-popup id="table-popup">
				<wc-div-table .cards=${this.sample.cards}> </wc-div-table>
			</wc-base-popup>`;
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
		iconButtonStyles(),
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
				height: 500px;

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
				width: 96px;
				height: 96px;
				padding: 0;
				margin: 0;
			}
		`,
	];
}

function iconButtonStyles() {
	return css`
		.icon-button {
			/* Focus rings */
			--sl-focus-ring-color: rgb(105, 208, 255);
			--sl-focus-ring-style: solid;
			--sl-focus-ring-width: 3px;
			--sl-focus-ring: var(--sl-focus-ring-style) var(--sl-focus-ring-width) var(--sl-focus-ring-color);
			--sl-focus-ring-offset: 1px;

			display: inline-block;
			color: rgb(142, 142, 154);

			flex: 0 0 auto;
			display: flex;
			align-items: center;
			background: none;
			border: none;
			border-radius: var(--sl-border-radius-medium);
			font-size: inherit;
			color: inherit;
			padding: var(--sl-spacing-x-small);
			cursor: pointer;
			transition: var(--sl-transition-x-fast) color;
			-webkit-appearance: none;
		}

		.icon-button:hover:not(.icon-button--disabled),
		.icon-button:focus-visible:not(.icon-button--disabled) {
			color: rgb(39, 186, 253);
		}

		.icon-button:active:not(.icon-button--disabled) {
			color: rgb(105, 208, 255);
		}

		.icon-button:focus {
			outline: none;
		}

		.icon-button--disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}

		.icon-button:focus-visible {
			outline: var(--sl-focus-ring);
			outline-offset: var(--sl-focus-ring-offset);
		}

		.icon-button__icon {
			pointer-events: none;
		}
	`;
}
