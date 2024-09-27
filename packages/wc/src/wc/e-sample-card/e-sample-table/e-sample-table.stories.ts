import { html } from 'lit';
import { cards } from './data';
import './e-sample-table';

export default {
	title: 'Elements/e-sample-card/e-sample-table',
};

export const Default = {
	render() {
		return html`<e-sample-table .cards=${cards}></e-sample-table>`;
	},
};
