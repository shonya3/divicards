import { html, css, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import { property, state, query } from 'lit/decorators.js';
import { OrderTriangleElement } from '../order-triangle';
import { toOrderedBy } from '@divicards/shared/toOrderedBy';
import '@shoelace-style/shoelace/dist/components/checkbox/checkbox.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/range/range.js';
import 'poe-custom-elements/item-card.js';
import { Column, DivinationCardRecord, Order } from '@divicards/shared/types';
import { styles } from './div-table.styles';

declare global {
	interface HTMLElementTagNameMap {
		'wc-div-table': DivTableElement;
	}
}

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

export interface Events {
	'min-price-changed': number;
	'column-order-changed': { column: Column; order: Order };
}

export class DivTableElement extends BaseElement {
	static override get defineList() {
		return [OrderTriangleElement];
	}
	static override tag = 'wc-div-table';
	static override styles = [styles];

	@property({ type: Array }) cards: Readonly<DivinationCardRecord[]> = [];
	@property({ reflect: true, type: Number, attribute: 'min-price' }) minPrice: number = 0;
	@property({ reflect: true, attribute: 'column' }) column: Column = 'sum';
	@property({ reflect: true, attribute: 'order' }) order: Order = 'desc';

	@state() _cards: DivinationCardRecord[] = [];
	@state() nameQuery = '';
	@state() hideZeroSum = false;
	@state() filteredRecords: DivinationCardRecord[] = [];
	@state() summary: { amount: number; sum: number } = Object.create({});

	@query('sl-checkbox#hide-zero-sum-checkbox') checkboxHideZeroSum!: HTMLInputElement;

	override willUpdate(map: PropertyValues<this>) {
		if (map.has('cards')) {
			this._cards = Array.from(this.cards);
		}

		if (map.has('cards') || map.has('column') || map.has('minPrice') || map.has('nameQuery') || map.has('order')) {
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

	protected override render() {
		return html`<div class="layout">
			<!-- Header -->
			<header class="header">
				<label for="filter-card-name">Enter name</label>
				<sl-input
					autofocus
					type="text"
					id="filter-card-name"
					.value=${this.nameQuery}
					@input=${this.#onNameQueryInput}
				></sl-input>

				<label class="slider-box">
					<label for="min-price-slider">min price </label>
					<sl-range
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
							<th class="th"><div>&numero;</div></th>
							<th class="th">
								<div>
									<span> Amount </span>
									<wc-order-triangle
										?active=${this.column === 'amount'}
										order=${this.column === 'amount' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('amount')}
									></wc-order-triangle>
								</div>
							</th>
							<th class="th">
								<div>
									<span> Name </span>
									<wc-order-triangle
										?active=${this.column === 'name'}
										order=${this.column === 'name' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('name')}
									></wc-order-triangle>
								</div>
							</th>
							<th class="th">
								<div>
									<span> Price </span>
									<wc-order-triangle
										?active=${this.column === 'price'}
										order=${this.column === 'price' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('price')}
									></wc-order-triangle>
								</div>
							</th>
							<th class="th">
								<div>
									<span> Sum </span>
									<wc-order-triangle
										?active=${this.column === 'sum'}
										order=${this.column === 'sum' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('sum')}
									></wc-order-triangle>
								</div>
							</th>
							<th class="th">
								<div>
									<span> Weight </span>
									<wc-order-triangle
										?active=${this.column === 'weight'}
										order=${this.column === 'weight' ? this.order : 'unordered'}
										@click=${() => this.#onOrderTriangleClicked('weight')}
									></wc-order-triangle>
								</div>
							</th>
						</tr>
					</thead>
					<tbody>
						${this.filteredRecords.map(({ amount, name, price, sum, weight }, index) => {
							return html`<tr>
								<td class="td">${index + 1}</td>
								<td class="td">${amount}</td>
								<td class="td td-name">
									<div><poe-item-card .name=${name}></poe-item-card> ${name}</div>
								</td>
								<td class="td">${price === null ? 'no price' : format(price)}</td>
								<td class="td">${format(sum ?? 0)}</td>
								<td class="td">${format(weight ?? 0)}</td>
							</tr>`;
						})}
					</tbody>
				</table>
			</div>
		</div>`;
	}

	toggleOrder(newActivecolumn: Column) {
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
		const target = e.composedPath()[0];
		if (target instanceof HTMLInputElement) {
			this.nameQuery = target.value;
		}
	}

	#onHideZeroCheckbox() {
		this.hideZeroSum = this.checkboxHideZeroSum.checked;
	}

	#onMinPriceSlider(e: InputEvent) {
		const target = e.composedPath()[0];
		if (target instanceof HTMLInputElement) {
			this.minPrice = Number(target.value);
			this.emit<Events['min-price-changed']>('min-price-changed', this.minPrice);
		}
	}

	#onOrderTriangleClicked(newActiveColumn: Column) {
		this.toggleOrder(newActiveColumn);
		this.emit<Events['column-order-changed']>('column-order-changed', {
			column: this.column,
			order: this.order,
		});
	}
}
