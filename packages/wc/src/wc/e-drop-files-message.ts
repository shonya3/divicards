import { html, css, LitElement, TemplateResult, CSSResult } from 'lit';
import './e-help-tip';
import { customElement } from 'lit/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-drop-files-message': DropFilesMessageElement;
	}
}

/**
 * Message to drop files for main app screen when dragging
 */
@customElement('e-drop-files-message')
export class DropFilesMessageElement extends LitElement {
	static override styles: CSSResult = css`
		:host {
			display: block;
		}
		.drop {
			font-size: 3rem;
		}

		.drop > span {
			color: deeppink;
			font-weight: 700;
		}
	`;

	protected override render(): TemplateResult {
		return html`<div style="display: flex; gap: 1rem">
			<div class="drop">Drop files <span>Here!</span></div>
		</div>`;
	}
}
