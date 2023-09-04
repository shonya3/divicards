import { Order } from '@divicards/shared/types';

export type Column = 'price' | 'amount' | 'sum' | 'name' | 'weight';
export type SortState = {
	[col in Column]: Order;
};
