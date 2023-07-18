import { html, css, PropertyValues } from 'lit';
import { BaseElement } from '../base-element';
import { property, state, query } from 'lit/decorators.js';
import { OrderTriangleElement } from '../order-triangle';
import { toOrderedBy } from './toOrderedBy';
import { Column, SortState } from './types';
import { DivinationCardRecord, Order } from '@divicards/shared/types';

declare global {
	interface HTMLElementTagNameMap {
		'wc-div-table': DivTableElement;
	}
}

const styles = css`
	:host {
		display: block;
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
		gap: 2rem;
		/* --col-name-width: 650px; */
		/* --col-width: calc((100% - var(--col-name-width)) / 4); */
		height: 100%;
		max-width: 1200px;

		color: var(--color);
		background-color: var(--bg-color);
	}

	.table {
		width: 100%;
	}

	.header {
		display: flex;
		gap: 1rem;
		align-items: center;
		flex-wrap: wrap;
	}

	tr {
		display: grid;
		grid-template-columns: 0.5fr 1.2fr 3fr 1fr 1fr 1fr;
	}

	.table > thead > tr > th {
		display: flex;
		gap: 0.5rem;
	}

	.column__name {
		overflow-x: hidden;
		white-space: nowrap;
		/* font-size: 18px; */
		/* min-width: 50px; */
	}
`;

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

export class DivTableElement extends BaseElement {
	static define(tag = 'wc-div-table'): void {
		if (!customElements.get(tag)) {
			customElements.define(tag, DivTableElement);
			OrderTriangleElement.define();
		}
	}
	static htmlTag = 'wc-div-table';
	static styles = [this.baseStyles, styles];

	@property({ type: Array }) cards: Readonly<DivinationCardRecord[]> = [];
	@property({ reflect: true, type: Number, attribute: 'min-price' }) minPrice: number = 0;
	@property({ reflect: true, attribute: 'active-column' }) activeColumn: Column = 'sum';
	@property({ reflect: true, attribute: 'active-column-order' }) activeColumnOrder: Order = 'desc';

	@state() protected _cards: DivinationCardRecord[] = [];
	@state() nameQuery = '';
	@state() hideZeroSum = false;

	@query('input#hide-zero-sum-checkbox') checkboxHideZeroSum!: HTMLInputElement;

	willUpdate(map: PropertyValues<this>) {
		if (map.has('cards')) {
			this._cards = Array.from(this.cards);
		}

		const needsToOrder =
			map.has('activeColumn') || map.has('minPrice') || map.has('nameQuery') || map.has('activeColumnOrder');
		if (needsToOrder) {
			this._cards = toOrderedBy(this._cards, this.activeColumn, this.activeColumnOrder);
		}
	}

	toggleOrder(newActivecolumn: Column) {
		if (this.activeColumn === newActivecolumn) {
			this.activeColumnOrder = this.activeColumnOrder === 'asc' ? 'desc' : 'asc';
		}
		// if column is unordered
		else {
			// if by name, start from A. Otherwise, start from the bigger values
			this.activeColumnOrder = newActivecolumn === 'name' ? 'asc' : 'desc';
		}

		this.activeColumn = newActivecolumn;
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

	render() {
		return html`<div class="table-container">${this.header()}${this.table()}</div>`;
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
		}
	}

	header() {
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
				<label for="hide-zero-sum-checkbox">hide names with zero sum</label>
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

	table() {
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

	thead() {
		return html`<thead>
			<tr>
				<th>&numero;</th>
				<th>
					<span class="column__name"> Amount </span>
					<wc-order-triangle
						?active=${this.activeColumn === 'amount'}
						order=${this.activeColumn === 'amount' ? this.activeColumnOrder : 'unordered'}
						@click=${() => this.toggleOrder('amount')}
					></wc-order-triangle>
				</th>
				<th>
					<span class="column__name"> Name </span>
					<wc-order-triangle
						?active=${this.activeColumn === 'name'}
						order=${this.activeColumn === 'name' ? this.activeColumnOrder : 'unordered'}
						@click=${() => this.toggleOrder('name')}
					></wc-order-triangle>
				</th>
				<th>
					<span class="column__name"> Price </span>
					<wc-order-triangle
						?active=${this.activeColumn === 'price'}
						order=${this.activeColumn === 'price' ? this.activeColumnOrder : 'unordered'}
						@click=${() => this.toggleOrder('price')}
					></wc-order-triangle>
				</th>
				<th>
					<span class="column__name"> Sum </span>
					<wc-order-triangle
						?active=${this.activeColumn === 'sum'}
						order=${this.activeColumn === 'sum' ? this.activeColumnOrder : 'unordered'}
						@click=${() => this.toggleOrder('sum')}
					></wc-order-triangle>
				</th>
				<th>
					<span class="column__name"> Weight </span>
				</th>
			</tr>
		</thead>`;
	}

	rows() {
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
