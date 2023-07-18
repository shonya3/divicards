import { DivinationCardRecord } from '@divicards/shared/types';
import { DivTableElement } from './wc/div-table/div-table';
import { cards as dummyCards } from './wc/div-table/dummy';
import { OrderTriangleElement } from './wc/order-triangle';
OrderTriangleElement.define();
DivTableElement.define();

console.log(dummyCards[0].name);

const cards: DivinationCardRecord[] = new Proxy(dummyCards, {
	set(target, p, newValue, receiver) {
		// console.log('Mutatung cards');

		return Reflect.set(target, p, newValue, receiver);
	},
});

const table = document.querySelector('wc-div-table')!;
table.cards = cards;
// table.toggleOrder('amount');

setTimeout(() => {
	console.log(dummyCards[0].name);
}, 3500);
