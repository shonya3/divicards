import { html } from 'lit';
import { DropFilesMessageElement } from './drop-files-message';
import { Meta } from '@storybook/web-components';

export default {
	title: 'Elements/drop-files-message',
} satisfies Meta<DropFilesMessageElement>;

export const Default = {
	render() {
		DropFilesMessageElement.define();
		return html`<wc-drop-files-message></wc-drop-files-message>`;
	},
};
