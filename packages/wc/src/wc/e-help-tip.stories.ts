import { html, TemplateResult } from 'lit';
import { Meta } from '@storybook/web-components';
import { HelpTipElement } from './e-help-tip.js';
import './e-help-tip';
import { styleMap } from 'lit/directives/style-map.js';

const meta: Meta<HelpTipElement> = {
	title: 'Elements/e-help-tip',
};
export default meta;

export const Default = {
	render(): TemplateResult {
		const styles = styleMap({ 'margin-left': '500px' });
		return html`<e-help-tip style=${styles}>
			<p>Excel, .csv or just .txt</p>
			<p>Required headers: name and amount</p>
			<img src="/simple.png" alt="Example of simple .txt file"
		/></e-help-tip>`;
	},
};
