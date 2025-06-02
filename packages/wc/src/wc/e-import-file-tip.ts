import { LitElement, CSSResult, css, TemplateResult, html } from 'lit';
import { customElement } from 'lit/decorators.js';
import './e-help-tip.js';
import { DefineComponent } from 'vue';

/**
 * @summary A help tip specifically for file import instructions.
 */
@customElement('e-import-file-tip')
export class ImportFileTipElement extends LitElement {
	static override styles: CSSResult = css`
		:host {
			display: inline-flex; /* Aligns icon with text if used inline */
			align-items: center;
			gap: 0.5rem; /* Space between text and icon */
		}
		.hint-text {
			font-size: 1em;
			color: var(--sl-color-neutral-700);
		}
	`;

	protected override render(): TemplateResult {
		return html`
			<span class="hint-text">or drag & drop files</span>
			<e-help-tip>
				<p>Excel, .csv or just .txt</p>
				<p>Required headers: name and amount</p>
				<img src="/simple.png" alt="Example of simple .txt file" />
			</e-help-tip>
		`;
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'e-import-file-tip': ImportFileTipElement;
	}
}

declare module 'vue' {
	interface GlobalComponents {
		'e-import-file-tip': DefineComponent<{}>;
	}
}
