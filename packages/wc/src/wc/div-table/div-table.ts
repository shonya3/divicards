import { html, css, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import { property, state, query } from 'lit/decorators.js';
import { OrderTriangleElement } from '../order-triangle';
import { toOrderedBy } from './toOrderedBy';
import { Column } from './types';
import { DivinationCardRecord, Order } from '@divicards/shared/types';

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
	static defineList = [OrderTriangleElement];
	static override tag = 'wc-div-table';
	static override styles = [this.baseStyles, styles()];

	@property({ type: Array }) cards: Readonly<DivinationCardRecord[]> = [];
	@property({ reflect: true, type: Number, attribute: 'min-price' }) minPrice: number = 0;
	@property({ reflect: true, attribute: 'column' }) column: Column = 'sum';
	@property({ reflect: true, attribute: 'order' }) order: Order = 'desc';

	@state() protected _cards: DivinationCardRecord[] = [];
	@state() nameQuery = '';
	@state() hideZeroSum = false;

	@query('input#hide-zero-sum-checkbox') checkboxHideZeroSum!: HTMLInputElement;

	override willUpdate(map: PropertyValues<this>) {
		if (map.has('cards')) {
			this._cards = Array.from(this.cards);
		}

		const needToOrder =
			map.has('cards') || map.has('column') || map.has('minPrice') || map.has('nameQuery') || map.has('order');
		if (needToOrder) {
			this._cards = toOrderedBy(this._cards, this.column, this.order);
		}
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

	get filteredRecords(): DivinationCardRecord[] {
		return this._cards.filter(({ name, price, sum }) => {
			if (this.hideZeroSum) {
				if (sum === 0 || sum === null) return false;
			}

			return name.toLowerCase().includes(this.nameQuery.trim().toLowerCase()) && (price ?? 0) >= this.minPrice;
		});
	}

	get summary(): { amount: number; sum: number } {
		let sum = 0;
		let amount = 0;

		for (const record of this.filteredRecords) {
			sum += record.sum ?? 0;
			amount += record.amount ?? 0;
		}

		return { amount, sum };
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

	protected override render() {
		return html`<div class="table-container">${this.header()}${this.table()}</div>`;
	}

	protected header() {
		return html`<header class="header">
			<label for="filter-card-name">Enter name</label>
			<input
				autofocus
				type="text"
				id="filter-card-name"
				.value=${this.nameQuery}
				@input=${this.#onNameQueryInput}
			/>
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
				<input
					class="slider"
					type="range"
					name=""
					id="min-price-slider"
					min="0"
					max="500"
					.value=${String(this.minPrice)}
					@input=${this.#onMinPriceSlider}
				/>
				<span class="ch-3">${this.minPrice}</span>
				<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />
			</label>
			<div style="display: flex; gap: 0.8rem">
				<label for="hide-zero-sum-checkbox">hide nullish rows</label>
				<input
					type="checkbox"
					name=""
					id="hide-zero-sum-checkbox"
					.checked=${this.hideZeroSum}
					@change=${this.#onHideZeroCheckbox}
				/>
			</div>
			<!-- <div>download filtered file</div> -->
		</header>`;
	}

	protected table() {
		return html`<table class="table">
			<colgroup>
				<col span="1" class="col" />
				<col span="1" class="col" />
				<col span="1" class="col-name" />
				<col span="1" class="col" />
				<col span="1" class="col" />
				<col span="1" class="col" />
			</colgroup>
			${this.thead()}
			<tbody>
				${this.rows()}
			</tbody>
		</table>`;
	}

	protected thead() {
		return html`<thead>
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
				</th>
			</tr>
		</thead>`;
	}

	protected rows() {
		return this.filteredRecords.map(({ amount, name, price, sum, weight }, index) => {
			return html`<tr>
				<td class="row">${index + 1}</td>
				<td class="row">${amount}</td>
				<td class="name-row">${name}</td>
				<td class="row">${format(price ?? 0)}</td>
				<td class="row">${format(sum ?? 0)}</td>
				<td class="row">${format(weight)}</td>
			</tr>`;
		});
	}
}

function styles() {
	return css`
		:host {
			display: block;
			max-width: 1220px;
			min-height: 100vh;
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
		.table-container {
			display: flex;
			flex-direction: column;
			height: 100%;
			max-width: 1220px;

			color: var(--color);
			background-color: var(--bg-color);
		}

		.header {
			position: sticky;
			top: 0;
			display: flex;
			gap: 1rem;
			align-items: center;
			flex-wrap: wrap;
			z-index: 2;
			background-color: var(--bg-color);
			padding-top: 20px;
			padding-bottom: 20px;
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
				box-shadow: rgba(0, 0, 0, 0.17) 0px -23px 25px 0px inset, rgba(0, 0, 0, 0.15) 0px -36px 30px 0px inset,
					rgba(0, 0, 0, 0.1) 0px -79px 40px 0px inset, rgba(0, 0, 0, 0.06) 0px 2px 1px 0px,
					rgba(0, 0, 0, 0.09) 0px 4px 2px 0px, rgba(0, 0, 0, 0.09) 0px 8px 4px 0px,
					rgba(0, 0, 0, 0.09) 0px 16px 8px 0px, rgba(0, 0, 0, 0.09) 0px 32px 16px 0px;
				filter: > .name-row {

				}
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
