import { html } from 'lit';
import { cards } from './data';
import './e-sample-table';

export default {
	title: 'Elements/div-table',
};

export const Default = {
	render() {
		return html`<e-sample-table .cards=${cards}></e-sample-table>`;
	},
};
