import { html } from 'lit';
import { Meta } from '@storybook/web-components';
import { BasePopupElement } from './base-popup';
import { DivTableElement } from './div-table/div-table';
import { cards } from './div-table/data';

export default {
	title: 'Elements/base-popup',
} satisfies Meta<BasePopupElement>;

export const Default = {
	render() {
		BasePopupElement.define();
		DivTableElement.define();

		const popup = document.createElement('wc-base-popup');
		popup.open = true;
		const table = document.createElement('wc-div-table');
		table.cards = cards;
		popup.append(table);

		return html`<button @click=${() => popup.showModal()}>Open</button>${popup}`;
	},
};
