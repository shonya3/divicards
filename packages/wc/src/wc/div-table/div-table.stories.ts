import { Meta } from '@storybook/web-components';
import { DivTableElement } from './div-table';
import { html } from 'lit';
import { cards } from './data';

export default {
	title: 'Elements/div-table',
};

export const Default = {
	render() {
		DivTableElement.define();
		return html`<wc-div-table .cards=${cards}></wc-div-table>`;
	},
};
