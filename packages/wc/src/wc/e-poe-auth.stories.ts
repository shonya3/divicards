import { Meta } from '@storybook/web-components';
import { PoeAuthElement } from './e-poe-auth.js';
import { html, TemplateResult } from 'lit';
import './e-poe-auth';

const meta: Meta<PoeAuthElement> = {
	title: 'Elements/e-poe-auth',
	args: { name: 'Chris', loggedIn: false },
	argTypes: {
		loggedIn: { control: { control: 'boolean' } },
		name: {
			control: { type: 'text' },
		},
	},
};
export default meta;

export const Default = {
	render({ name, loggedIn }: { name: string; loggedIn: boolean }): TemplateResult {
		return html`<e-poe-auth ?loggedIn=${loggedIn} .name=${name}></e-poe-auth>`;
	},
};
