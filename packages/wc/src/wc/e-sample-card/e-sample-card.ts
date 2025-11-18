import { html, LitElement, CSSResult, TemplateResult } from 'lit';
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
import '@shoelace-style/shoelace/dist/components/checkbox/checkbox.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import './e-sample-table/e-sample-table';
import '../e-base-popup';
import { SampleTableElement } from './e-sample-table/e-sample-table.js';
import { LeagueChangeEvent } from '../events/change/league.js';
import './e-form-export-sample/e-form-export-sample';
import type { TablePreferences } from '@divicards/shared/types.js';
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
	FilenameChangeEvent,
	Events,
} from './events.js';
import { VueEventHandlers } from '../../event-utils.js';

export type Props = {
	league?: TradeLeague;
	filename: string;
	selected: boolean | null;
	uuid: string;
	minimumCardPrice: number;
	sample: DivinationCardsSample;
	csvDataForDrag: string;
};

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

@customElement('e-sample-card')
export class SampleCardElement extends LitElement {
	static override styles: Array<CSSResult> = [styles];

	@property({ reflect: true }) league: TradeLeague = ACTIVE_LEAGUE;
	@property({ reflect: true }) filename: string = 'NO FILE NAMENO FILE NAME';
	@property({ type: Boolean, reflect: true }) selected: boolean | null = false;
	@property({ reflect: true }) uuid: string = 'NO ID';
	@property({ type: Number, reflect: true, attribute: 'minimum-card-price' }) minimum_card_price: number = 0;
	@property({ type: String, attribute: false }) csvDataForDrag = 'CSV DATA';
	@property({ type: Object }) sample: DivinationCardsSample = { notCards: [], fixedNames: [], cards: [] };

	@query('e-base-popup#table-popup') tablePopup!: BasePopupElement;
	@query('#selected-checkbox') selectedCheckbox!: HTMLInputElement;
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

	#handleDragStart(event: DragEvent) {
		if (!event.dataTransfer || !this.csvDataForDrag) {
			// If no CSV data is provided, or dataTransfer is not available, do nothing.
			// Optionally, you could prevent the drag or show a visual cue.
			if (!this.csvDataForDrag) {
				console.warn('Drag started, but no csvDataForDrag available for sample:', this.filename);
				event.preventDefault(); // Stop the drag if no data
			}
			return;
		}
		const csvContent = this.csvDataForDrag;
		let dragFilename = this.filename;
		if (!dragFilename.toLowerCase().endsWith('.csv')) {
			dragFilename = `${dragFilename}.csv`;
		}

		const dataUrl = `data:text/csv;charset=utf-8,${encodeURIComponent(csvContent)}`;
		event.dataTransfer.setData('DownloadURL', `text/csv:${dragFilename}:${dataUrl}`);
		event.dataTransfer.effectAllowed = 'copy';

		// Set the drag image to be the entire card element.
		// event.offsetX and event.offsetY position the drag image relative to the cursor.
		// This makes the card appear as if grabbed from where the cursor is on the handle.
		event.dataTransfer.setDragImage(this, event.offsetX, event.offsetY);
	}

	protected override render(): TemplateResult {
		const defaultSpreadsheetId = localStorage.getItem('sheets-spreadsheet-id') ?? '';
		const defaultSheetTitle = `Divicards Export - ${new Date().toLocaleString()}`;
		const defaultTablePreferences: TablePreferences = {
			order: 'desc',
			columns: new Set(['name', 'amount']),
			orderedBy: 'amount',
			cardsMustHaveAmount: true,
			minPrice: 0,
		};

		return html`<div
			class=${classMap({
				'sample-card': true,
				'sample-card--selected': Boolean(this.selected),
			})}
		>
			<div class="card-header">
				<div class="drag-handle-container" draggable="true" @dragstart=${this.#handleDragStart}>
					<sl-icon name="grip-vertical" class="drag-handle-icon" title="Drag to create file"></sl-icon>
					<span class="drag-handle-text">Drag to export</span>
				</div>
				<sl-icon-button @click=${this.#emit_delete_this_sample} class="btn-delete" name="x-lg"></sl-icon-button>
			</div>

				<e-base-popup id="form_popup">
					<e-form-export-sample
						.export_sample_to=${this.export_sample_to}
						.spreadsheetId=${defaultSpreadsheetId}
						.sheetTitle=${defaultSheetTitle}
						.table_preferences=${defaultTablePreferences}
						@sample__presubmit=${this.#handle_presubmit}
					></e-form-export-sample>
				</e-base-popup>

			<sl-input
				size="small"
				class="filename-input"
				value=${this.filename}
				@sl-change=${this.#handle_filename_change}
				.label=${`Edit filename`}
			></sl-input>

			<div class="minor-icons">
				${this.sample.fixedNames.length > 0
					? html`<e-fixed-names .fixedNames=${this.sample.fixedNames}></e-fixed-names>`
					: null}
				${this.sample.notCards.length > 0
					? html`<e-not-cards .notCards=${this.sample.notCards}></e-not-cards>`
					: null}
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
				<sl-button @click=${this.#emit_save_to_file_click}>
					<sl-icon style="font-size:1.6rem" name="filetype-csv"></sl-icon>
					Save to file</sl-button
				>
				<sl-button @click=${this.#emit_google_sheets_click}>
					<sl-icon style="font-size:1.6rem" name="file-earmark-spreadsheet"></sl-icon>
					Export to Google Sheets</sl-button
				>
			</div>
			${this.selected === null
				? null
				: html`
						<sl-checkbox
							id="selected-checkbox"
							.checked=${this.selected}
							@sl-change=${this.#change_selected_and_emit}
						></sl-checkbox>
				  `}
			<e-base-popup id="table-popup">
				<e-sample-table .cards=${this.sample.cards}> </e-sample-table>
			</e-base-popup>
		</div>`;
	}

	get filteredCards(): Array<DivinationCardRecord> {
		return (this.sample?.cards ?? []).filter(card => {
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
		this.dispatchEvent(new SaveToFileClickEvent(this.sample, this.league, this.filename));
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

	#handle_filename_change(e: Event): void {
		const newFilename = (e.target as HTMLInputElement).value;
		this.filename = newFilename;
		this.dispatchEvent(new FilenameChangeEvent(newFilename));
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
