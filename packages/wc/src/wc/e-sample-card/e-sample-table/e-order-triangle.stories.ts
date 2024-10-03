import { OrderTriangleElement, Size } from './e-order-triangle';
import { Meta } from '@storybook/web-components';
import { html } from 'lit';
import { Order } from '@divicards/shared/types';
import './e-order-triangle';

export default {
	title: 'Elements/e-sample-card/e-sample-table/e-order-triangle',
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
		return html`<e-order-triangle order=${order} size=${size} ?active=${active}></e-order-triangle>`;
	},
};
