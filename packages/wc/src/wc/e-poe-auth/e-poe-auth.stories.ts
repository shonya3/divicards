import { Meta } from '@storybook/web-components';
import { AuthState, PoeAuthElement } from './e-poe-auth.js';
import { html, TemplateResult } from 'lit';
import './e-poe-auth';

const meta: Meta<PoeAuthElement> = {
	title: 'Elements/e-poe-auth',
	args: { auth: { username: 'Chris', loggedIn: true } },
};
export default meta;

export const Default = {
	render(): TemplateResult {
		return html`<e-poe-auth></e-poe-auth>`;
	},
};

export const LoggedIn = {
	render({ auth }: { auth: AuthState }): TemplateResult {
		return html`<e-poe-auth .auth=${auth}></e-poe-auth>`;
	},
};
