import { Column, Order, CardRecord } from '../../types';

export const byPrice = (order: Order, records: CardRecord[]) => {
	return records.sort((a, b) => {
		if (order === 'asc') return a.calculated - b.calculated;
		if (order === 'desc') return b.calculated - a.calculated;
		throw new Error('Invalid order');
	});
};

export const byStackSize = (order: Order, records: CardRecord[]) => {
	return records.sort((a, b) => {
		if (order === 'asc') return a.stackSize - b.stackSize;
		if (order === 'desc') return b.stackSize - a.stackSize;
		throw new Error('invalid order');
	});
};

export const byTotal = (order: Order, records: CardRecord[]) => {
	return records.sort((a, b) => {
		if (order === 'asc') return a.total - b.total;
		if (order === 'desc') return b.total - a.total;
		throw new Error('invalid order');
	});
};

export const orderBy = (column: Column, order: Order, records: CardRecord[]): CardRecord[] => {
	switch (column) {
		case 'price':
			return byPrice(order, records);
		case 'stackSize':
			return byStackSize(order, records);
		case 'total':
			return byTotal(order, records);
		default:
			throw new Error('Invalid column name');
	}
};
