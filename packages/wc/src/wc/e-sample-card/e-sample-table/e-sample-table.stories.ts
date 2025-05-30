import { html, TemplateResult } from 'lit';
import { cards } from './data.js';
import './e-sample-table';

export default {
	title: 'Elements/e-sample-card/e-sample-table',
};

export const Default = {
	render(): TemplateResult {
		return html`<e-sample-table .cards=${cards}></e-sample-table>`;
	},
};
