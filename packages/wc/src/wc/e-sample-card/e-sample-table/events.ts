import { Column, Order } from '@divicards/shared/types.js';

export type Events = [typeof ChangeMinPrice, typeof ChangeColumnOrder];

declare global {
	interface HTMLElementEventMap {
		'sample-table__change:min_price': ChangeMinPrice;
	}
}
export class ChangeMinPrice extends Event {
	static readonly tag = 'sample-table__change:min_price';

	constructor(public readonly $min_price: number) {
		super(ChangeMinPrice.tag);
	}
}

declare global {
	interface HTMLElementEventMap {
		'sample-table__change:column-order': ChangeColumnOrder;
	}
}
export class ChangeColumnOrder extends Event {
	static readonly tag = 'sample-table__change:column-order';

	constructor(public readonly $column: Column, public readonly $order: Order) {
		super(ChangeColumnOrder.tag);
	}
}
