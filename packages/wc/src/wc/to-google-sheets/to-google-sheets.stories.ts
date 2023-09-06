import { html } from 'lit';
import { ToGoogleSheetsElement } from './to-google-sheets';
import { Meta } from '@storybook/web-components';

export default {
	title: 'Elements/to-google-sheets',
} satisfies Meta<ToGoogleSheetsElement>;

export const Default = {
	render() {
		ToGoogleSheetsElement.define();
		return html`<wc-to-google-sheets></wc-to-google-sheets>`;
	},
};
