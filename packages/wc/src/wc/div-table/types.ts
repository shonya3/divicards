import { Order } from '@divicards/shared/types';

export type Column = 'price' | 'amount' | 'sum' | 'name';
export type SortState = {
	[col in Column]: Order;
};
