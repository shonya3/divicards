import { html } from 'lit';
import { Meta } from '@storybook/web-components';
import { BasePopupElement } from './base-popup';
import { cards } from './sample-card/e-sample-table/data';
import './sample-card/e-sample-table/e-sample-table';

export default {
	title: 'Elements/base-popup',
} satisfies Meta<BasePopupElement>;

export const Default = {
	render() {
		BasePopupElement.define();

		const popup = document.createElement('wc-base-popup');
		popup.open = true;
		const table = document.createElement('e-sample-table');
		table.cards = cards;
		popup.append(table);

		return html`<button @click=${() => popup.showModal()}>Open</button>${popup}`;
	},
};
