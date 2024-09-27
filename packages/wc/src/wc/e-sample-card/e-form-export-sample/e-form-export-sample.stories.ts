import { html } from 'lit';
import { FormExportSampleElement } from './e-form-export-sample';
import './e-form-export-sample';
import { Meta } from '@storybook/web-components';

export default {
	title: 'Elements/e-sample-card/e-form-export-sample',
} satisfies Meta<FormExportSampleElement>;

export const Default = {
	render() {
		return html`<e-form-export-sample></e-form-export-sample>`;
	},
};
