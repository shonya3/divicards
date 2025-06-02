import { html, TemplateResult } from 'lit';
import './e-import-file-tip.js';
import { ImportFileTipElement } from './e-import-file-tip.js';
import { Meta } from '@storybook/web-components-vite';

const meta: Meta<ImportFileTipElement> = {
	title: 'Elements/e-import-file-tip',
	component: 'e-import-file-tip',
	argTypes: {},
};

export default meta;

export const Default = {
	render: (): TemplateResult => html`<e-import-file-tip></e-import-file-tip>`,
};
