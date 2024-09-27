import { Meta } from '@storybook/web-components';
import { PoeAuthElement } from './e-poe-auth';
import { html } from 'lit';
import './e-poe-auth';

export default {
	title: 'Elements/poe-auth',
	args: { name: 'Chris', loggedIn: false },
	argTypes: {
		loggedIn: { control: { control: 'boolean' } },
		name: {
			control: { type: 'text' },
		},
	},
} satisfies Meta<PoeAuthElement>;

export const Default = {
	render({ name, loggedIn }: { name: string; loggedIn: boolean }) {
		return html`<e-poe-auth ?loggedIn=${loggedIn} .name=${name}></e-poe-auth>`;
	},
};
