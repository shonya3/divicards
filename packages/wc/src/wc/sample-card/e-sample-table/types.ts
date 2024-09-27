import { Column, Order } from '@divicards/shared/types';

export type SortState = {
	[col in Column]: Order;
};
