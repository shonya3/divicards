import { Meta } from '@storybook/web-components';
NotCardsElement;
import { html } from 'lit';
import { NotCardsElement } from './not-cards';
import { notCards } from './data';

export default {
	title: 'Elements/file-card/not-cards',
} satisfies Meta<NotCardsElement>;

export const Default = {
	render() {
		NotCardsElement.define();
		return html`<wc-not-cards .notCards=${notCards}></wc-not-cards>`;
	},
};
