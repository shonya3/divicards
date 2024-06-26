import { html, css, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import { property, state, query } from 'lit/decorators.js';
import { OrderTriangleElement } from '../order-triangle';
import { toOrderedBy } from '@divicards/shared/toOrderedBy';
import '@shoelace-style/shoelace/dist/components/checkbox/checkbox.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/range/range.js';
import { Column, DivinationCardRecord, Order } from '@divicards/shared/types';

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
	static override styles = [styles()];

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
		return html`<div class="div-table">
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

				<span class="stats"
					>found
					<span class="ch-6">${this.filteredRecords.length} </span>
					card names
					<span class="ch-6">${this.summary.amount}</span>
					cards,
					<span class="ch-7">${format(this.summary.sum)}</span>
					<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />)</span
				>
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

			<!-- Table -->
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
						<th>&numero;</th>
						<th>
							<span class="column-name"> Amount </span>
							<wc-order-triangle
								?active=${this.column === 'amount'}
								order=${this.column === 'amount' ? this.order : 'unordered'}
								@click=${() => this.#onOrderTriangleClicked('amount')}
							></wc-order-triangle>
						</th>
						<th>
							<span class="column-name"> Name </span>
							<wc-order-triangle
								?active=${this.column === 'name'}
								order=${this.column === 'name' ? this.order : 'unordered'}
								@click=${() => this.#onOrderTriangleClicked('name')}
							></wc-order-triangle>
						</th>
						<th>
							<span class="column-name"> Price </span>
							<wc-order-triangle
								?active=${this.column === 'price'}
								order=${this.column === 'price' ? this.order : 'unordered'}
								@click=${() => this.#onOrderTriangleClicked('price')}
							></wc-order-triangle>
						</th>
						<th>
							<span class="column-name"> Sum </span>
							<wc-order-triangle
								?active=${this.column === 'sum'}
								order=${this.column === 'sum' ? this.order : 'unordered'}
								@click=${() => this.#onOrderTriangleClicked('sum')}
							></wc-order-triangle>
						</th>
						<th>
							<span class="column-name"> Weight </span>
							<wc-order-triangle
								?active=${this.column === 'weight'}
								order=${this.column === 'weight' ? this.order : 'unordered'}
								@click=${() => this.#onOrderTriangleClicked('weight')}
							></wc-order-triangle>
						</th>
					</tr>
				</thead>
				<tbody>
					${this.filteredRecords.map(({ amount, name, price, sum, weight }, index) => {
						return html`<tr>
							<td class="row">${index + 1}</td>
							<td class="row">${amount}</td>
							<td class="name-row">${name}</td>
							<td class="row">${price === null ? 'no price' : format(price)}</td>
							<td class="row">${format(sum ?? 0)}</td>
							<td class="row">${format(weight ?? 0)}</td>
						</tr>`;
					})}
				</tbody>
			</table>
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

function styles() {
	return css`
		:host {
			display: block;
			max-width: 1220px;
			min-height: 100vh;
			background-color: var(--bg-color, #242424);
		}

		.slider-box {
			display: flex;
			justify-content: center;
			align-items: center;
			gap: 0.5rem;
		}
		.stats {
			display: flex;
			align-items: center;
		}
		.div-table {
			display: flex;
			flex-direction: column;
			height: 100%;
			max-width: 1220px;

			color: var(--color, rgba(255, 255, 255, 0.87));
			background-color: var(--bg-color, #242424);
			padding: 1rem;
		}

		.header {
			position: sticky;
			top: 0;
			display: flex;
			gap: 1rem;
			align-items: center;
			flex-wrap: wrap;
			z-index: 2;
			background-color: var(--bg-color, #242424);
			padding-top: 20px;
			padding-bottom: 20px;
			border-bottom: 1px solid black;
		}

		.column-name {
			overflow-x: hidden;
			white-space: nowrap;
		}

		th {
			gap: 0.5rem;
			display: flex;
			align-items: center;
		}

		tbody > tr:first-of-type {
			margin-top: 0.25rem;
		}

		tr {
			display: grid;
			grid-template-columns: 0.5fr 1.2fr 3fr 1fr 1fr 1fr;

			&:hover {
				outline: 1px black solid;
				box-shadow:
					rgba(0, 0, 0, 0.17) 0px -23px 25px 0px inset,
					rgba(0, 0, 0, 0.15) 0px -36px 30px 0px inset,
					rgba(0, 0, 0, 0.1) 0px -79px 40px 0px inset,
					rgba(0, 0, 0, 0.06) 0px 2px 1px 0px,
					rgba(0, 0, 0, 0.09) 0px 4px 2px 0px,
					rgba(0, 0, 0, 0.09) 0px 8px 4px 0px,
					rgba(0, 0, 0, 0.09) 0px 16px 8px 0px,
					rgba(0, 0, 0, 0.09) 0px 32px 16px 0px;
			}
		}

		.ch-3 {
			/* display: block; */
			text-align: center;
			min-width: 3ch;
		}
		.ch-6 {
			text-align: center;
			min-width: 6ch;
		}
		.ch-7 {
			text-align: center;
			min-width: 7ch;
		}
	`;
}
