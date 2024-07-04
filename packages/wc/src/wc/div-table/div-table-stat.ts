import { LitElement, html, css, TemplateResult } from 'lit';
import { customElement } from 'lit/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-div-table-stat': DivTableStatElement;
	}
}

@customElement('wc-div-table-stat')
export class DivTableStatElement extends LitElement {
	protected render(): TemplateResult {
		return html`content`;
	}

	static styles = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
	`;
}
