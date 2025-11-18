import { html, css, nothing, LitElement, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/divider/divider.js';
import { Column, type TablePreferences } from '@divicards/shared/types.js';
import '../../e-help-tip';
// import { emit } from '../../../utils';

export type ExportSampleTo = 'file' | 'sheets';
const isColumn = (s: unknown): s is Column => {
	return s === 'name' || s === 'amount' || s === 'weight' || s === 'price';
};

export interface Props {
	spreadsheetId?: string;
	sheetTitle?: string;
	preferences: TablePreferences;
	export_sample_to: ExportSampleTo;
	error: string | null;
}

export interface Events {
	submit: Props;
}

@customElement('e-form-export-sample')
export class FormExportSampleElement extends LitElement {
	static override styles: Array<CSSResult> = [styles()];

	@property({ type: Object }) table_preferences: TablePreferences = {
		order: 'desc',
		columns: new Set(['name', 'amount']),
		orderedBy: 'amount',
		cardsMustHaveAmount: true,
		minPrice: 0,
	};
	@property({ reflect: true, attribute: 'spreadsheet-id' }) spreadsheetId: string = '';
	@property({ reflect: true, attribute: 'sheet-title' }) sheetTitle: string = '';
	@property({ attribute: false, reflect: true }) error: string | null = null;
	@property({ reflect: true }) export_sample_to: ExportSampleTo = 'sheets';

	#applyPreset(preset: 'minimal' | 'full' | 'detailed') {
		if (preset === 'minimal') {
			this.table_preferences.columns = new Set(['name', 'amount']);
			this.table_preferences.orderedBy = 'amount';
			this.table_preferences.order = 'desc';
		} else if (preset === 'full') {
			this.table_preferences.columns = new Set(['name', 'amount', 'price', 'sum']);
			this.table_preferences.orderedBy = 'amount';
			this.table_preferences.order = 'desc';
		} else {
			this.table_preferences.columns = new Set(['name', 'amount', 'weight', 'price', 'sum']);
			this.table_preferences.orderedBy = 'amount';
			this.table_preferences.order = 'desc';
		}
	}

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

		this.dispatchEvent(
			new PresubmitExportFormEvent({
				spreadsheetId: this.spreadsheetId,
				sheetTitle: this.sheetTitle,
				preferences: this.table_preferences,
				export_sample_to: this.export_sample_to,
				error: this.error,
			})
		);
	}
	protected override render(): TemplateResult {
		return html`<div id="root">
			<div class="header">
				<div class="title">Divination Cards Export</div>
				<small class="subtitle">This export targets divination cards only</small>
			</div>
			${this.export_sample_to === 'sheets' ? this.sheetsFieldset() : nothing}
			<sl-divider></sl-divider>
			<form @submit=${this.#onSubmit} id="form">
				<fieldset style="margin-top: 0.5rem">
					<legend>Table Preferences</legend>
					<fieldset id="fieldset-presets">
						<legend>Presets</legend>
						<div class="presets">
							<sl-button size="small" @click=${() => this.#applyPreset('minimal')}>Minimal</sl-button>
							<sl-button size="small" @click=${() => this.#applyPreset('full')}>Full</sl-button>
							<sl-button size="small" @click=${() => this.#applyPreset('detailed')}>Detailed</sl-button>
						</div>
					</fieldset>
					<fieldset id="fieldset-hide-nullish">
						<legend>Cards must have the amount</legend>
						<div class="input-wrapper">
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
						<div class="input-wrapper">
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
						<div class="input-wrapper">
							<label for="asc">Smallest to Largest (A -> Z)</label>
							<input
								id="asc"
								type="radio"
								name="order"
								.checked=${this.table_preferences.order === 'asc'}
								@input=${this.#onOrderRadio}
							/>
						</div>
						<div class="input-wrapper">
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
						<div class="input-wrapper">
							<div>
								<label for="column-name">name</label>
								<input
									id="column-name"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('name')}
									disabled
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'name')}
								/>
							</div>

							<div class="input-wrapper">
								<label for="columnd-amount">amount</label>
								<input
									id="columnd-amount"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('amount')}
									disabled
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'amount')}
								/>
							</div>

							<div class="input-wrapper">
								<label for="column-weight">weight</label>
								<input
									id="column-weight"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('weight')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'weight')}
								/>
							</div>

							<div class="input-wrapper">
								<label for="column-price">price</label>
								<input
									id="column-price"
									type="checkbox"
									.checked=${this.table_preferences.columns.has('price')}
									@input=${(e: InputEvent) => this.#onColumnCheckbox(e, 'price')}
								/>
							</div>

							<div class="input-wrapper">
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
						<div class="input-wrapper">
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

	protected sheetsFieldset(): TemplateResult {
		return html`
                <fieldset>
					<legend>Sheets Identificators</legend>
					<fieldset id="fieldset-spreadsheetId">
						<legend>Spreadsheet Id</legend>
						<div class="input-wrapper">
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

		.header {
			display: flex;
			flex-direction: column;
			gap: 0.2rem;
			margin-bottom: 0.4rem;
		}
		.title {
			font-weight: 600;
		}
		.subtitle {
			opacity: 0.8;
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

		.input-wrapper {
			display: flex;
			align-items: center;
			gap: 0.25rem;
		}

		.presets {
			display: flex;
			gap: 0.4rem;
			align-items: center;
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'e-form-export-sample': FormExportSampleElement;
	}
}

declare global {
	interface HTMLElementEventMap {
		sample__presubmit: PresubmitExportFormEvent;
	}
}
export interface ExportFormArgs {
	spreadsheetId?: string;
	sheetTitle?: string;
	preferences: TablePreferences;
	export_sample_to: ExportSampleTo;
	error: string | null;
}
export class PresubmitExportFormEvent extends Event {
	readonly export_sample_to: ExportSampleTo;
	readonly spreadsheetId?: string;
	readonly sheetTitle?: string;
	readonly preferences: TablePreferences;
	readonly error: string | null = null;

	constructor(
		{ spreadsheetId, sheetTitle, preferences, export_sample_to, error }: ExportFormArgs,
		tag = 'sample__presubmit',
		options?: EventInit
	) {
		super(tag, options);
		this.spreadsheetId = spreadsheetId;
		this.sheetTitle = sheetTitle;
		this.preferences = preferences;
		this.export_sample_to = export_sample_to;
		this.error = error;
	}
}
