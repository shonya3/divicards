import { OrderTriangleElement, Size } from './e-order-triangle.js';
import { Meta } from '@storybook/web-components';
import { html, TemplateResult } from 'lit';
import { Order } from '@divicards/shared/types.js';
import './e-order-triangle';

const meta: Meta<OrderTriangleElement> = {
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
};
export default meta;

export const Default = {
	render({ order, active, size }: { order: Order; active: boolean; size: Size }): TemplateResult {
		return html`<e-order-triangle order=${order} size=${size} ?active=${active}></e-order-triangle>`;
	},
};
