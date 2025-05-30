import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement } from 'lit/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-sample-table-stat': DivTableStatElement;
	}
}

@customElement('e-sample-table-stat')
export class DivTableStatElement extends LitElement {
	protected render(): TemplateResult {
		return html`content`;
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
	`;
}
