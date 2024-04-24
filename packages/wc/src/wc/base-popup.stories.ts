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

		return html`<wc-base-popup open><wc-div-table .cards=${cards}></wc-div-table></wc-base-popup>`;
	},
};
