import { html } from 'lit';
import { DropFilesMessageElement } from './e-drop-files-message.js';
import { Meta } from '@storybook/web-components';
import './e-drop-files-message';

export default {
	title: 'Elements/drop-files-message',
} satisfies Meta<DropFilesMessageElement>;

export const Default = {
	render() {
		return html`<e-drop-files-message></e-drop-files-message>`;
	},
};
