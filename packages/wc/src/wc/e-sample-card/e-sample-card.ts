import { html, nothing, LitElement, CSSResult, TemplateResult } from 'lit';
import { DefineComponent } from 'vue';
import { LeagueSelectElement } from '../e-league-select.js';
import '../e-league-select';
import { BasePopupElement } from '../e-base-popup.js';
import './e-fixed-names/e-fixed-names';
import './e-not-cards/e-not-cards';
import {
	DivinationCardRecord,
	type DivinationCardsSample,
	type TradeLeague,
	isTradeLeague,
} from '@divicards/shared/types.js';
import { customElement, property, query, state } from 'lit/decorators.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib.js';
import { classMap } from 'lit/directives/class-map.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/range/range.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import SlRange from '@shoelace-style/shoelace/dist/components/range/range.js';
import './e-sample-table/e-sample-table';
import '../e-base-popup';
import { SampleTableElement } from './e-sample-table/e-sample-table.js';
import { LeagueChangeEvent } from '../events/change/league.js';
import './e-form-export-sample/e-form-export-sample';
import '../e-base-popup';
import { type ExportSampleTo, PresubmitExportFormEvent } from './e-form-export-sample/e-form-export-sample.js';
import { styles } from './e-sample-card.styles.js';
import {
	SubmitExportSampleEvent,
	SaveToFileClickEvent,
	GoogleSheetsClickEvent,
	SelectedChangeEvent,
	DeleteThisSampleEvent,
	MinimumCardsPriceChangeEvent,
	Events,
} from './events.js';
import { VueEventHandlers } from '../../event-utils.js';

export interface Props {
	league?: TradeLeague;
	filename: string;
	selected: boolean | null;
	uuid: string;
	minimumCardPrice: number;
	sample: DivinationCardsSample;
}

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

@customElement('e-sample-card')
export class SampleCardElement extends LitElement {
	static override styles: Array<CSSResult> = [styles];

	@property({ reflect: true }) league: TradeLeague = ACTIVE_LEAGUE;
	@property({ reflect: true }) filename: string = 'NO FILE NAMENO FILE NAME';
	@property({ type: Boolean, reflect: true }) selected: boolean | null = false;
	@property({ reflect: true }) uuid: string = 'NO ID';
	@property({ type: Number, reflect: true, attribute: 'minimum-card-price' }) minimum_card_price: number = 0;
	@property({ type: Object }) sample: DivinationCardsSample = { notCards: [], fixedNames: [], cards: [] };

	@query('e-base-popup#table-popup') tablePopup!: BasePopupElement;
	@query('input#selected-checkbox') selectedCheckbox!: HTMLInputElement;
	@query('e-league-select') leagueSelect!: LeagueSelectElement;
	@query('#minimum-card-price-slider') priceSlider!: HTMLInputElement;
	@query('e-sample-table') table!: SampleTableElement;
	@query('sl-range') rangeEl!: SlRange;
	/** Export sample form popup. */
	@query('#form_popup') form_popup!: BasePopupElement;

	@state() export_sample_form_state = {};
	/** Export the cards sample to file or to google sheets */
	@state() export_sample_to: ExportSampleTo = 'file';

	constructor() {
		super();
		this.addEventListener('sample__save-to-file-click', () => {
			this.export_sample_to = 'file';
			this.form_popup.open = true;
		});
		this.addEventListener('sample__google-sheets-click', () => {
			this.export_sample_to = 'sheets';
			this.form_popup.open = true;
		});
	}

	protected override render(): TemplateResult {
		return html`<div
			class=${classMap({
				'sample-card': true,
				'sample-card--selected': Boolean(this.selected),
			})}
		>
			<e-base-popup id="form_popup">
				<e-form-export-sample
					.export_sample_to=${this.export_sample_to}
					@sample__presubmit=${this.#handle_presubmit}
				></e-form-export-sample>
			</e-base-popup>

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
				.value=${this.minimum_card_price}
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
				<sl-button size="large" @click=${this.#emit_save_to_file_click}>
					<sl-icon style="font-size:1.6rem" name="filetype-csv"></sl-icon>
					Save to file</sl-button
				>
				<sl-button @click=${this.#emit_google_sheets_click} size="large">
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
						@change=${this.#change_selected_and_emit}
				  />`}
			<e-base-popup id="table-popup">
				<e-sample-table .cards=${this.sample.cards}> </e-sample-table>
			</e-base-popup>
		</div>`;
	}

	get filteredCards(): Array<DivinationCardRecord> {
		return this.sample.cards.filter(card => {
			return (card.price ?? 0) >= this.minimum_card_price;
		});
	}

	get filteredSummary(): { amount: number; value: number } {
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

	#handle_presubmit(e: PresubmitExportFormEvent) {
		this.dispatchEvent(
			new SubmitExportSampleEvent({
				form_args: {
					error: e.error,
					export_sample_to: e.export_sample_to,
					preferences: e.preferences,
					sheetTitle: e.sheetTitle,
					spreadsheetId: e.spreadsheetId,
				},
				league: this.league,
				sample: this.sample,
				filename: this.filename,
			})
		);
	}

	#emit_save_to_file_click() {
		this.dispatchEvent(
			new SaveToFileClickEvent({
				sample: this.sample,
				league: this.league,
				filename: this.filename,
			})
		);
	}

	#emit_google_sheets_click(): void {
		this.dispatchEvent(new GoogleSheetsClickEvent(this.sample, this.league));
	}

	#openSampleTablePopup(): void {
		this.tablePopup.showModal();
	}

	#change_selected_and_emit() {
		if (this.selected === null) return;
		this.selected = this.selectedCheckbox.checked;
		this.dispatchEvent(new SelectedChangeEvent(this.selected));
	}

	#handle_league_change(e: LeagueChangeEvent): void {
		if (!isTradeLeague(e.$league)) {
			return;
		}
		this.league = e.$league;
		this.dispatchEvent(new LeagueChangeEvent(e.$league));
	}

	#emit_delete_this_sample(): void {
		this.dispatchEvent(new DeleteThisSampleEvent(this.uuid));
	}

	#onMinPriceRange(e: Event): void {
		if (!(e.target && 'value' in e.target)) {
			return;
		}
		this.minimum_card_price = Number(e.target.value);
		this.dispatchEvent(new MinimumCardsPriceChangeEvent(this.minimum_card_price));
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'e-sample-card': SampleCardElement;
	}
}

declare module 'vue' {
	interface GlobalComponents {
		'e-sample-card': DefineComponent<Props & VueEventHandlers<Events>>;
	}
}
