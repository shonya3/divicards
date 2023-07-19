import { DivinationCardRecord } from '@divicards/shared/types';
import { DivTableElement } from './wc/div-table/div-table';
import { cards } from './wc/div-table/div-table.props';
import { OrderTriangleElement } from './wc/order-triangle';
OrderTriangleElement.define();
DivTableElement.define();

const table = document.querySelector('wc-div-table')!;
table.cards = cards;

table.addEventListener('column-order-changed', e => {
	console.log(e.type, e.detail);
});

table.addEventListener('min-price-changed', e => {
	console.log(e.type, e.detail);
});
