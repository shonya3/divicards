import { html } from 'lit';
import { Meta } from '@storybook/web-components';
import { HelpTipElement } from './help-tip';
import { styleMap } from 'lit/directives/style-map.js';

export default {
	title: 'Elements/help-tip',
} satisfies Meta<HelpTipElement>;

export const Default = {
	render() {
		const styles = styleMap({ 'margin-left': '500px' });
		HelpTipElement.define();
		return html`<wc-help-tip style=${styles}>
			<p>Excel, .csv or just .txt</p>
			<p>Required headers: name and amount</p>
			<img src="/simple.png" alt="Example of simple .txt file"
		/></wc-help-tip>`;
	},
};
