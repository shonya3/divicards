import { html } from 'lit';
import { FormExportSampleElement } from './form-export-sample';
import { Meta } from '@storybook/web-components';

export default {
	title: 'Elements/sample-card/e-form-export-sample',
} satisfies Meta<FormExportSampleElement>;

export const Default = {
	render() {
		FormExportSampleElement.define();
		return html`<wc-form-export-sample></wc-form-export-sample>`;
	},
};
