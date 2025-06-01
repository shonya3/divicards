import { Meta } from '@storybook/web-components-vite';
import { html, TemplateResult } from 'lit';
import { NotCardsElement } from './e-not-cards.js';
import './e-not-cards';
import { notCards } from './data.js';

const meta: Meta<NotCardsElement> = {
	title: 'Elements/e-sample-card/e-not-cards',
};
export default meta;

export const Default = {
	render(): TemplateResult {
		return html`<e-not-cards .notCards=${notCards}></e-not-cards>`;
	},
};
