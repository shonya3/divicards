import { Meta } from '@storybook/web-components';
import { PoeAuthElement } from './poe-auth';
import { html } from 'lit';
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
		PoeAuthElement.define();
		return html`<wc-poe-auth ?loggedIn=${loggedIn} .name=${name}></wc-poe-auth>`;
	},
};
