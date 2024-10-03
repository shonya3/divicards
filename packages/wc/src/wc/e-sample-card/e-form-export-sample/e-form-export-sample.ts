import { html, css, nothing, LitElement } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import { Column, TablePreferences } from '@divicards/shared/types';
import '../../e-help-tip';
import { emit } from '../../../utils';

export type To = 'file' | 'sheets';
const isColumn = (s: unknown): s is Column => {
	return s === 'name' || s === 'amount' || s === 'weight' || s === 'price';
};

export interface Props {
	spreadsheetId: string;
	sheetTitle: string;
	preferences: TablePreferences;
	to: To;
	error: string | null;
}

export interface Events {
	submit: Props;
}

@customElement('e-form-export-sample')
export class FormExportSampleElement extends LitElement {
	static override styles = [styles()];

	@property({ type: Object }) table_preferences: TablePreferences = {
		order: 'desc',
		columns: new Set(['name', 'amount']),
		orderedBy: 'amount',
		cardsMustHaveAmount: false,
		minPrice: 0,
	};
	@property({ reflect: true, attribute: 'spreadsheet-id' }) spreadsheetId: string = '';
	@property({ reflect: true, attribute: 'sheet-title' }) sheetTitle: string = '';
	@property({ attribute: false, reflect: true }) error: string | null = null;
	@property({ reflect: true }) to: To = 'sheets';

	#onColumnCheckbox(e: InputEvent, column: Column) {
		if (!(e.target instanceof HTMLInputElement)) {
			return;
		}

		if (e.target.checked) {
			this.table_preferences.columns.add(column);
		} else {
			this.table_preferences.columns.delete(column);
		}
	}

	#onOrderRadio(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLInputElement)) return;

		if (target.id === 'asc') {
			this.table_preferences.order = 'asc';
		} else this.table_preferences.order = 'desc';
	}

	#onOrderedBySelected(e: InputEvent) {
		const target = e.target;
		if (!(target instanceof HTMLSelectElement)) return;
		if (!isColumn(target.value)) return;

		this.table_preferences.orderedBy = target.value;
	}

	#onCardsMustHaveAmountCheckbox(e: InputEvent) {
		if (!(e.target instanceof HTMLInputElement)) {
			return;
		}

		this.table_preferences.cardsMustHaveAmount = e.target.checked;
	}

	#onMinPriceSlider(e: InputEvent) {
		if (!(e.target instanceof HTMLInputElement)) {
			return;
		}

		this.table_preferences.minPrice = Number(e.target.value);
	}

	#onSheetTitleInput(e: InputEvent) {
		if (!(e.target instanceof HTMLInputElement)) return;
		this.error = null;
		this.sheetTitle = e.target.value;
	}

	#onSpreadsheetIdInput(e: InputEvent) {
		if (!(e.target instanceof HTMLInputElement)) return;
		this.error = null;
		this.spreadsheetId = e.target.value;
	}

	#onSubmit(e: SubmitEvent) {
		e.preventDefault();

		const props: Props = {
			spreadsheetId: this.spreadsheetId,
			sheetTitle: this.sheetTitle,
			preferences: this.table_preferences,
			to: this.to,
			error: this.error,
		};

		emit<Events['submit']>(this, 'submit', props);
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
								.checked=${this.table_preferences.cardsMustHaveAmount}
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
								.value=${this.table_preferences.orderedBy}
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
								.checked=${this.table_preferences.order === 'asc'}
								@input=${this.#onOrderRadio}
							/>
						</div>
						<div>
							<label for="desc">Largest to Smallest (Z -> A)</label>
							<input
								id="desc"
								type="radio"
								name="order"
								.checked=${this.table_preferences.order === 'desc'}
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
									.checked=${this.table_preferences.columns.has('name')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'name')}
								/>
							</div>

							<div>
								<label for="columnd-amount">amount</label>
								<input
									id="columnd-amount"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('amount')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'amount')}
								/>
							</div>

							<div>
								<label for="column-weight">weight</label>
								<input
									id="column-weight"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('weight')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'weight')}
								/>
							</div>

							<div>
								<label for="column-price">price</label>
								<input
									id="column-price"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('price')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'price')}
								/>
							</div>

							<div>
								<label for="column-sum">sum</label>
								<input
									id="column-sum"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('sum')}
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
								.value=${String(this.table_preferences.minPrice)}
								@input=${this.#onMinPriceSlider}
							/>
						</div>
					</fieldset>
				</fieldset>
				${this.error && html`<div id="error">${this.error}</div>`}
				<sl-button type="submit" id="submit">Submit</sl-button>
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
							<e-help-tip>
								<img src="spreadsheetid.png" alt="screenshot of google sheet url">
									https://docs.google.com/spreadsheets/d/1sDXpbG2bkqrOYScnvjMXTTg718dEc0LMDVHzllbAgmM/edit#gid=562350311
								</img>

								<p>spreadsheet id here is: 1sDXpbG2bkqrOYScnvjMXTTg718dEc0LMDVHzllbAgmM</p>
							</e-help-tip>
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

declare global {
	interface HTMLElementTagNameMap {
		'e-form-export-sample': FormExportSampleElement;
	}
}
