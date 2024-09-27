import { Meta } from '@storybook/web-components';
NotCardsElement;
import { html } from 'lit';
import { NotCardsElement } from './e-not-cards';
import './e-not-cards';
import { notCards } from './data';

export default {
	title: 'Elements/e-sample-card/e-not-cards',
} satisfies Meta<NotCardsElement>;

export const Default = {
	render() {
		return html`<e-not-cards .notCards=${notCards}></e-not-cards>`;
	},
};
