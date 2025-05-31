import { html, PropertyValues, LitElement, CSSResult, TemplateResult } from 'lit';
import { property, state, query, customElement } from 'lit/decorators.js';
import { toOrderedBy } from '@divicards/shared/toOrderedBy.js';
import '@shoelace-style/shoelace/dist/components/checkbox/checkbox.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/range/range.js';
import 'poe-custom-elements/item-card.js';
import { type Column, DivinationCardRecord, type Order } from '@divicards/shared/types.js';
import { styles } from './e-sample-table.styles.js';
import './e-order-triangle';
import { ChangeColumnOrder, ChangeMinPrice } from './events.js';

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

@customElement('e-sample-table')
export class SampleTableElement extends LitElement {
	static override styles: Array<CSSResult> = [styles];

	@property({ type: Array }) cards: Readonly<DivinationCardRecord[]> = [];
	@property({ reflect: true, type: Number, attribute: 'min-price' }) minPrice: number = 0;
	@property({ reflect: true, attribute: 'column' }) column: Column = 'sum';
	@property({ reflect: true, attribute: 'order' }) order: Order = 'desc';

	@state() _cards: DivinationCardRecord[] = [];
	@state() nameQuery = '';
	@state() hideZeroSum = true;
	@state() filteredRecords: DivinationCardRecord[] = [];
	@state() summary: { amount: number; sum: number } = Object.create({});

	@query('sl-checkbox#hide-zero-sum-checkbox') checkboxHideZeroSum!: HTMLInputElement;

	override willUpdate(map: PropertyValues<this>): void {
		if (map.has('cards')) {
			this._cards = Array.from(this.cards);
		}

		if (
			map.has('cards') ||
			map.has('column') ||
			map.has('minPrice') ||
			map.has('nameQuery') ||
			map.has('order') ||
			map.has('hideZeroSum')
		) {
			this._cards = toOrderedBy(this._cards, this.column, this.order);

			this.filteredRecords = this._cards.filter(({ name, price, sum }) => {
				if (this.hideZeroSum) {
					if (sum === 0 || sum === null) return false;
				}

				return (
					name.toLowerCase().includes(this.nameQuery.trim().toLowerCase()) && (price ?? 0) >= this.minPrice
				);
			});

			this.summary = this.filteredRecords.reduce(
				(summary, record) => {
					summary.sum += record.sum ?? 0;
					summary.amount += record.amount ?? 0;
					return summary;
				},
				{ sum: 0, amount: 0 }
			);
		}
	}

	protected override render(): TemplateResult {
		return html`<div class="layout">
			<!-- Header -->
			<header class="header">
				<sl-input
					label="Enter name"
					autofocus
					type="text"
					id="filter-card-name"
					.value=${this.nameQuery}
					@input=${this.#onNameQueryInput}
				></sl-input>

				<label class="slider-box">
					<sl-range
						label="Min price"
						id="min-price-slider"
						min="0"
						max="500"
						.value=${this.minPrice}
						@input=${this.#onMinPriceSlider}
					></sl-range>
					<span class="ch-3">${this.minPrice}</span>
					<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />
				</label>
				<div style="display: flex; gap: 0.8rem">
					<sl-checkbox
						id="hide-zero-sum-checkbox"
						.checked=${this.hideZeroSum}
						@sl-change=${this.#onHideZeroCheckbox}
						>hide nullish rows</sl-checkbox
					>
				</div>
			</header>

			<ul class="stats">
				<li>
					<div class="stat">
						<img width="40" height="40" class="chaos-img" src="/chaos.png" alt="chaos" />
						${format(this.summary.sum)}
					</div>
				</li>
				<li>
					<div class="stat">
						<img width="40" height="40" src="/divination-card.png" alt="Divination card" />
						${this.summary.amount}
					</div>
				</li>
				<li>
					<div class="stat">
						<!-- <img width="40" height="40" src="/divination-card.png" alt="Divination card" /> -->
						<span>names</span>
						${this.filteredRecords.length}
					</div>
				</li>
			</ul>

			<!-- Table -->
			<div class="table-wrapper">
				<table class="table">
					<colgroup>
						<col span="1" class="col" />
						<col span="1" class="col" />
						<col span="1" class="col-name" />
						<col span="1" class="col" />
						<col span="1" class="col" />
						<col span="1" class="col" />
					</colgroup>
					<thead>
						<tr>
							<th class="th th-number"><div>&numero;</div></th>
							<th class="th th-amount">
								<div>
									<span> Amount </span>
									<e-order-triangle
										?active=${this.column === 'amount'}
										order=${this.column === 'amount' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('amount')}
									></e-order-triangle>
								</div>
							</th>
							<th class="th th-name">
								<div>
									<span> Name </span>
									<e-order-triangle
										?active=${this.column === 'name'}
										order=${this.column === 'name' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('name')}
									></e-order-triangle>
								</div>
							</th>
							<th class="th th-price">
								<div>
									<span> Price </span>
									<e-order-triangle
										?active=${this.column === 'price'}
										order=${this.column === 'price' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('price')}
									></e-order-triangle>
								</div>
							</th>
							<th class="th th-sum">
								<div>
									<span> Sum </span>
									<e-order-triangle
										?active=${this.column === 'sum'}
										order=${this.column === 'sum' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('sum')}
									></e-order-triangle>
								</div>
							</th>
							<th class="th th-weight">
								<div>
									<span> Weight </span>
									<e-order-triangle
										?active=${this.column === 'weight'}
										order=${this.column === 'weight' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('weight')}
									></e-order-triangle>
								</div>
							</th>
						</tr>
					</thead>
					<tbody>
						${this.filteredRecords.map(({ amount, name, price, sum, weight }, index) => {
							return html`<tr>
								<td class="td td-number">${index + 1}</td>
								<td class="td td-amount">${amount}</td>
								<td class="td td-name">
									<div><poe-item-card .name=${name}></poe-item-card> ${name}</div>
								</td>
								<td class="td td-price">${price === null ? 'no price' : format(price)}</td>
								<td class="td td-sum">${format(sum ?? 0)}</td>
								<td class="td td-weight">${format(weight ?? 0)}</td>
							</tr>`;
						})}
					</tbody>
				</table>
			</div>
		</div>`;
	}

	toggleOrder(newActivecolumn: Column): void {
		if (this.column === newActivecolumn) {
			this.order = this.order === 'asc' ? 'desc' : 'asc';
		}
		// if column is unordered
		else {
			// if by name, start from A. Otherwise, start from the bigger values
			this.order = newActivecolumn === 'name' ? 'asc' : 'desc';
		}

		this.column = newActivecolumn;
	}

	#onNameQueryInput(e: InputEvent) {
		const target = e.target as HTMLInputElement;
		this.nameQuery = target.value;
	}

	#onHideZeroCheckbox() {
		this.hideZeroSum = this.checkboxHideZeroSum.checked;
	}

	#onMinPriceSlider(e: InputEvent) {
		const target = e.target as HTMLInputElement;
		this.minPrice = Number(target.value);
		this.dispatchEvent(new ChangeMinPrice(this.minPrice));
	}

	#onOrderTriangleClicked(newActiveColumn: Column) {
		this.toggleOrder(newActiveColumn);
		this.dispatchEvent(new ChangeColumnOrder(this.column, this.order));
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'e-sample-table': SampleTableElement;
	}
}
