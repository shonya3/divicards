import { Order } from '../../types';

export type Column = 'price' | 'amount' | 'sum';
export type SortState = {
	[col in Column]: Order;
} & { activeColumn: Column };
