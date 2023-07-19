import { Meta } from '@storybook/web-components';
import { DivTableElement } from './div-table';
import { divTableProps } from './div-table.props';
import { html } from 'lit';
import { cards } from './div-table.props';

export default {
	title: 'Elements/div-table',
};

export const Default = {
	render() {
		DivTableElement.define();
		return html`<wc-div-table .cards=${cards}></wc-div-table>`;
	},
};
