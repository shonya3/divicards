import { html, TemplateResult } from 'lit';
import { Meta } from '@storybook/web-components-vite';
import { HelpTipElement } from './e-help-tip.js';
import './e-help-tip';

const meta: Meta<HelpTipElement> = {
	title: 'Elements/e-help-tip',
};
export default meta;

export const Default = {
	render(): TemplateResult {
		return html`<e-help-tip>
			<p>Excel, .csv or just .txt</p>
			<p>Required headers: name and amount</p>
			<img src="/simple.png" alt="Example of simple .txt file"
		/></e-help-tip>`;
	},
};
