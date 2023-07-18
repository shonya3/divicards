import { DivinationCardRecord, Order } from '@divicards/shared/types';
import { Column } from './types';

export const byPrice = (order: Order, cards: DivinationCardRecord[]) => {
	return Array.from(cards).sort((a, b) => {
		if (order === 'asc') return (a.price ?? 0) - (b.price ?? 0);
		if (order === 'desc') return (b.price ?? 0) - (a.price ?? 0);
		throw new Error('Invalid order');
	});
};

export const byAmount = (order: Order, cards: DivinationCardRecord[]) => {
	return Array.from(cards).sort((a, b) => {
		if (order === 'asc') return a.amount - b.amount;
		if (order === 'desc') return b.amount - a.amount;
		throw new Error('invalid order');
	});
};

export const bySum = (order: Order, cards: DivinationCardRecord[]) => {
	return Array.from(cards).sort((a, b) => {
		if (order === 'asc') return (a.sum ?? 0) - (b.sum ?? 0);
		if (order === 'desc') return (b.sum ?? 0) - (a.sum ?? 0);
		throw new Error('invalid order');
	});
};

export const byName = (order: Order, cards: DivinationCardRecord[]) => {
	return Array.from(cards).sort((a, b) => {
		if (order === 'asc') return a.name < b.name ? -1 : 1;
		if (order === 'desc') return a.name > b.name ? -1 : 1;
		throw new Error('invalid order');
	});
};

export const toOrderedBy = (cards: DivinationCardRecord[], column: Column, order: Order): DivinationCardRecord[] => {
	if (order === 'unordered') return Array.from(cards);
	switch (column) {
		case 'name':
			return byName(order, cards);
		case 'price':
			return byPrice(order, cards);
		case 'amount':
			return byAmount(order, cards);
		case 'sum':
			return bySum(order, cards);
		default:
			throw new Error('Invalid column name');
	}
};
