import { Meta } from '@storybook/web-components';
import { PaginationElement } from './e-pagination';
import './e-pagination';
import { html } from 'lit';

export default {
	title: 'Elements/e-pagination',
} satisfies Meta<PaginationElement>;

export const Default = {
	render() {
		return html`<e-pagination .n=${50}></e-pagination>`;
	},
};
