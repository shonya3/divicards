import { Column, Order } from '@divicards/shared/types.js';
import { EventMapFrom } from '../../../event-utils.js';

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export type Events = [typeof ChangeMinPrice, typeof ChangeColumnOrder];

export class ChangeMinPrice extends Event {
	static readonly tag = 'sample-table__change:min_price';

	constructor(public readonly $min_price: number) {
		super(ChangeMinPrice.tag);
	}
}

export class ChangeColumnOrder extends Event {
	static readonly tag = 'sample-table__change:column-order';

	constructor(public readonly $column: Column, public readonly $order: Order) {
		super(ChangeColumnOrder.tag);
	}
}
