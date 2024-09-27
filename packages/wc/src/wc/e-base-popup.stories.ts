import { html } from 'lit';
import { Meta } from '@storybook/web-components';
import { BasePopupElement } from './e-base-popup';
import './e-base-popup';
import { cards } from './e-sample-card/e-sample-table/data';
import './e-sample-card/e-sample-table/e-sample-table';

export default {
	title: 'Elements/e-base-popup',
} satisfies Meta<BasePopupElement>;

export const Default = {
	render() {
		const popup = document.createElement('e-base-popup');
		popup.open = true;
		const table = document.createElement('e-sample-table');
		table.cards = cards;
		popup.append(table);

		return html`<button @click=${() => popup.showModal()}>Open</button>${popup}`;
	},
};
