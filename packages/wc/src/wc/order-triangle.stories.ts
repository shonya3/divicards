import { OrderTriangleElement, Size } from './order-triangle';
import { Meta } from '@storybook/web-components';
import { html } from 'lit';
import { Order } from '@divicards/shared/types';

export default {
	title: 'Elements/order-triangle',
	args: {
		order: 'asc',
		active: false,
		size: '16px',
	},
	argTypes: {
		order: {
			options: ['asc', 'desc', 'unordered'],
			control: { type: 'radio' },
			size: { type: 'text' },
		},
		active: { control: { control: 'boolean' } },
	},
} satisfies Meta<OrderTriangleElement>;

export const Default = {
	render({ order, active, size }: { order: Order; active: boolean; size: Size }) {
		OrderTriangleElement.define();
		return html`<wc-order-triangle order=${order} size=${size} ?active=${active}></wc-order-triangle>`;
	},
};
