import { Meta } from '@storybook/web-components-vite';
import { AuthState, ButtonSize, PoeAuthElement } from './e-poe-auth.js';
import { html, TemplateResult } from 'lit';
import './e-poe-auth.js';

const meta: Meta<PoeAuthElement> = {
	title: 'Elements/e-poe-auth',
	args: { auth: { username: 'Chris#0000', loggedIn: true }, size: 'small' },
	argTypes: {
		size: {
			options: ['small', 'medium', 'large'],
			control: 'radio',
		},
	},
};
export default meta;

export const Default = {
	render({ size }: { size: ButtonSize }): TemplateResult {
		return html`<e-poe-auth .size=${size}></e-poe-auth>`;
	},
};

export const LoggedIn = {
	render({ auth, size }: { auth: AuthState; size: ButtonSize }): TemplateResult {
		return html`<e-poe-auth .size=${size} .auth=${auth}></e-poe-auth>`;
	},
};
