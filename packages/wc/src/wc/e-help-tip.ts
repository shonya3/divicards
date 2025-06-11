import { css, CSSResult, html, LitElement, TemplateResult } from 'lit';
import { customElement } from 'lit/decorators.js';
import './e-simple-tooltip.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-help-tip': HelpTipElement;
	}
}

/**
 * A questionmark icon with hoverable tip content
 * @slot  The tip's main content
 */
@customElement('e-help-tip')
export class HelpTipElement extends LitElement {
	static override styles: Array<CSSResult> = [styles()];
	override render(): TemplateResult {
		return html`<div class="icon-trigger">
				<sl-icon name="question-circle"></sl-icon>
			</div>
			<e-simple-tooltip part="tooltip">
				<slot></slot>
			</e-simple-tooltip>`;
	}
}

function styles() {
	return css`
		:host {
			display: inline-block;
		}

		.icon-trigger {
			color: var(--sl-color-sky-700);
			font-size: 1.25rem;
			display: flex;
		}
	`;
}
