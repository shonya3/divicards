import { html, css, nothing } from 'lit';
import { BaseElement } from '../base-element';
import { property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import { Column, Order, TablePreferences } from '@divicards/shared/types';
import { HelpTipElement } from '../help-tip';

export type To = 'file' | 'sheets';
const isColumn = (s: unknown): s is Column => {
	return s === 'name' || s === 'amount' || s === 'weight' || s === 'price';
};

declare global {
	interface HTMLElementTagNameMap {
		'wc-form-export-sample': FormExportSampleElement;
	}
}

export interface Props {
	spreadsheetId: string;
	sheetTitle: string;
	order: Order;
	orderedBy: Column;
	columns: Set<Column>;
	cardsMustHaveAmount: boolean;
	minPrice: number;
	to: To;
	error: string | null;
}

export interface Events {
	'upd:columns': FormExportSampleElement['columns'];
	'upd:order': FormExportSampleElement['order'];
	'upd:orderedBy': FormExportSampleElement['orderedBy'];
	'upd:cardsMustHaveAmount': FormExportSampleElement['cardsMustHaveAmount'];
	'upd:minPrice': FormExportSampleElement['minPrice'];
	'upd:sheetTitle': FormExportSampleElement['sheetTitle'];
	'upd:tablePreferences': TablePreferences;
	'upd:spreadsheetId': FormExportSampleElement['spreadsheetId'];
	submit: Props;
}

export class FormExportSampleElement extends BaseElement {
	static override get defineList() {
		return [HelpTipElement];
	}
	static override tag = 'wc-form-export-sample';
	static override styles = [styles()];

	@property({ reflect: true, attribute: 'spreadsheet-id' }) spreadsheetId: string = '';
	@property({ reflect: true, attribute: 'sheet-title' }) sheetTitle: string = '';
	@property({ reflect: true }) order: Order = 'desc';
	@property({ reflect: true }) orderedBy: Column = 'amount';
	@property({ attribute: false }) columns: Set<Column> = new Set(['name', 'amount']);
	@property({ type: Boolean }) cardsMustHaveAmount: boolean = false;
	@property({ type: Number }) minPrice = 0;
	@property({ attribute: false, reflect: true }) error: string | null = null;
	@property({ reflect: true }) to: To = 'sheets';

	get tablePreferences(): TablePreferences {
		return {
			order: this.order,
			columns: this.columns,
			orderedBy: this.orderedBy,
			cardsMustHaveAmount: this.cardsMustHaveAmount,
			minPrice: this.minPrice,
		};
	}

	#onColumnCheckbox(e: InputEvent, column: Column) {
		const target = e.target;
		if (!(target instanceof HTMLInputElement)) return;

		const checked = target.checked;
		if (checked) {
			this.columns.add(column);
		} else {
			this.columns.delete(column);
		}

		console.log('#onColumnsCheckbox', this.columns);

		this.emit<Events['upd:columns']>('upd:columns', this.columns);
		this.emit<Events['upd:tablePreferences']>('upd:tablePreferences', this.tablePreferences);
	}

	#onOrderRadio(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLInputElement)) return;

		if (target.id === 'asc') {
			this.order = 'asc';
		} else this.order = 'desc';

		this.emit<Events['upd:order']>('upd:order', this.order);
		this.emit<Events['upd:tablePreferences']>('upd:tablePreferences', this.tablePreferences);
	}

	#onOrderedBySelected(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLSelectElement)) return;
		if (!isColumn(target.value)) return;

		this.orderedBy = target.value;
		this.emit<Events['upd:orderedBy']>('upd:orderedBy', this.orderedBy);
		this.emit<Events['upd:tablePreferences']>('upd:tablePreferences', this.tablePreferences);
	}

	#onCardsMustHaveAmountCheckbox(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLInputElement)) return;

		this.cardsMustHaveAmount = target.checked;
		this.emit<Events['upd:cardsMustHaveAmount']>('upd:cardsMustHaveAmount', this.cardsMustHaveAmount);
		this.emit<Events['upd:tablePreferences']>('upd:tablePreferences', this.tablePreferences);
	}

	#onMinPriceSlider(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLInputElement)) return;

		this.minPrice = Number(target.value);

		this.emit<Events['upd:minPrice']>('upd:minPrice', this.minPrice);
		this.emit<Events['upd:tablePreferences']>('upd:tablePreferences', this.tablePreferences);
	}

	#onSheetTitleInput(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLInputElement)) return;
		this.error = null;

		this.sheetTitle = target.value;
		this.emit<Events['upd:sheetTitle']>('upd:sheetTitle', this.sheetTitle);
	}

	#onSpreadsheetIdInput(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLInputElement)) return;
		this.error = null;

		this.spreadsheetId = target.value;
		this.emit<Events['upd:spreadsheetId']>('upd:spreadsheetId', this.spreadsheetId);
	}

	#onSubmit(e: SubmitEvent) {
		e.preventDefault();

		const props: Props = {
			spreadsheetId: this.spreadsheetId,
			sheetTitle: this.sheetTitle,
			order: this.order,
			orderedBy: this.orderedBy,
			columns: this.columns,
			cardsMustHaveAmount: this.cardsMustHaveAmount,
			minPrice: this.minPrice,
			to: this.to,
			error: this.error,
		};

		this.emit<Events['submit']>('submit', props);
	}
	protected override render() {
		return html`<div id="root">
			${this.to === 'sheets' ? this.sheetsFieldset() : nothing}
			<form @submit=${this.#onSubmit} id="form">
				<fieldset style="margin-top: 0.5rem">
					<legend>Table Preferences</legend>
					<fieldset id="fieldset-hide-nullish">
						<legend>Cards must have the amount</legend>
						<div>
							<label for="input-title">Interested only in cards with amount</label>
							<input
								id="input-title"
								type="checkbox"
								.checked=${this.cardsMustHaveAmount}
								@input=${this.#onCardsMustHaveAmountCheckbox}
							/>
						</div>
					</fieldset>
					<fieldset id="fieldset-order">
						<legend>Order</legend>
						<div>
							<label for="ordered-by">Ordered by</label>
							<select
								@input=${this.#onOrderedBySelected}
								.value=${this.orderedBy}
								name=""
								id="ordered-by"
							>
								<option value="name">name</option>
								<option value="amount">amount</option>
								<option value="weight">weight</option>
								<option value="price">price</option>
								<option value="price">sum</option>
							</select>
						</div>
						<div>
							<label for="asc">Smallest to Largest (A -> Z)</label>
							<input
								id="asc"
								type="radio"
								name="order"
								.checked=${this.order === 'asc'}
								@input=${this.#onOrderRadio}
							/>
						</div>
						<div>
							<label for="desc">Largest to Smallest (Z -> A)</label>
							<input
								id="desc"
								type="radio"
								name="order"
								.checked=${this.order === 'desc'}
								@input=${this.#onOrderRadio}
							/>
						</div>
					</fieldset>

					<fieldset id="fieldset-columns">
						<legend>Columns</legend>
						<div>
							<div>
								<label for="column-name">name</label>
								<input
									id="column-name"
									type="checkbox"
									.checked=${this.columns.has('name')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'name')}
								/>
							</div>

							<div>
								<label for="columnd-amount">amount</label>
								<input
									id="columnd-amount"
									type="checkbox"
									.checked=${this.columns.has('amount')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'amount')}
								/>
							</div>

							<div>
								<label for="column-weight">weight</label>
								<input
									id="column-weight"
									type="checkbox"
									.checked=${this.columns.has('weight')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'weight')}
								/>
							</div>

							<div>
								<label for="column-price">price</label>
								<input
									id="column-price"
									type="checkbox"
									.checked=${this.columns.has('price')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'price')}
								/>
							</div>

							<div>
								<label for="column-sum">sum</label>
								<input
									id="column-sum"
									type="checkbox"
									.checked=${this.columns.has('sum')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'sum')}
								/>
							</div>
						</div>
					</fieldset>

					<fieldset id="fieldset-min-price">
						<legend>Minimum chaos price</legend>
						<div>
							<label for="input-min-price">Ignore cheaper cards</label>
							<input
								id="input-min-price"
								type="number"
								min="0"
								max="10000"
								.value=${String(this.minPrice)}
								@input=${this.#onMinPriceSlider}
							/>
						</div>
					</fieldset>
				</fieldset>
				${this.error && html`<div id="error">${this.error}</div>`}
				<sl-button id="submit">Submit</sl-button>
			</form>
		</div>`;
	}

	protected sheetsFieldset() {
		return html`
                <fieldset>
					<legend>Sheets Identificators</legend>
					<fieldset id="fieldset-spreadsheetId">
						<legend>Spreadsheet Id</legend>
						<div>
							<label for="input-spreadsheet-id">id</label>
							<input
								required
								size="50"
								id="input-spreadsheet-id"
								type="text"
								.value=${this.spreadsheetId}
								@input=${this.#onSpreadsheetIdInput}
							/>
							<wc-help-tip>
								<img src="spreadsheetid.png" alt="screenshot of google sheet url">
									https://docs.google.com/spreadsheets/d/1sDXpbG2bkqrOYScnvjMXTTg718dEc0LMDVHzllbAgmM/edit#gid=562350311
								</img>

								<p>spreadsheet id here is: 1sDXpbG2bkqrOYScnvjMXTTg718dEc0LMDVHzllbAgmM</p>
							</wc-help-tip>
						</div>
					</fieldset>

					<fieldset id="fieldset-title">
						<legend>Name</legend>
						<div>
							<label for="input-title">Sheet title</label>
							<input
								required
								id="input-title"
								type="text"
								.value=${this.sheetTitle}
								@input=${this.#onSheetTitleInput}
							/>
						</div>
					</fieldset>
				</fieldset>`;
	}
}

function styles() {
	return css`
		#root {
			padding: 2rem;
			width: 600px;
		}

		#submit {
			margin-top: 1rem;
			display: block;
			margin-inline: auto;
			font-size: 2rem;
			border: 1px solid #fff;
		}

		#error {
			margin-top: 1rem;
			border: 2px solid red;
		}

		wc-help-tip::part(tooltip) {
			left: -520px;
			font-size: 0.7rem;
		}
	`;
}
