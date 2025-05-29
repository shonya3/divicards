import { Column, Order } from '@divicards/shared/types.js';

export type SortState = {
	[col in Column]: Order;
};
