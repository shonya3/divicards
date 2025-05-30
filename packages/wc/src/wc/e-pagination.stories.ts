import { Meta } from '@storybook/web-components';
import { PaginationElement } from './e-pagination.js';
import './e-pagination';
import { html, TemplateResult } from 'lit';

const meta: Meta<PaginationElement> = {
	title: 'Elements/e-pagination',
};
export default meta;

export const Default = {
	render(): TemplateResult {
		return html`<e-pagination .n=${50}></e-pagination>`;
	},
};
