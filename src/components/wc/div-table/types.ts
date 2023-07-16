import { Order } from '../../../types';

export type Column = 'price' | 'amount' | 'sum' | 'name';
export type SortState = {
	[col in Column]: Order;
} & { activeColumn: Column };
