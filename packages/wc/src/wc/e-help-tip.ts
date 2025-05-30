import { css, CSSResult, html, LitElement, TemplateResult } from 'lit';
import { customElement } from 'lit/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-help-tip': HelpTipElement;
	}
}

/**
 * @summary A questionmark logo with hoverable tip content
 * @slot  The tip's main content
 */
@customElement('e-help-tip')
export class HelpTipElement extends LitElement {
	static override styles: Array<CSSResult> = [styles()];
	override render(): TemplateResult {
		return html`<div class="help-tip">
			<div part="tooltip" class="tooltip">
				<slot> Your tip here </slot>
			</div>
		</div>`;
	}
}

function styles() {
	return css`
		:host {
			display: inline-block;
		}
		.help-tip {
			position: relative;
			text-align: center;
			background-color: deeppink;
			border-radius: 50%;
			width: 24px;
			height: 24px;
			line-height: 26px;
			cursor: default;
		}

		.help-tip:before {
			content: '?';
			font-weight: bold;
			color: #fff;
		}

		.help-tip:hover .tooltip {
			display: block;
			transform-origin: 100% 0%;

			-webkit-animation: fadeIn 0.3s ease-in-out;
			animation: fadeIn 0.3s ease-in-out;
		}

		.tooltip {
			/* The tooltip */
			display: none;
			text-align: left;
			background-color: var(--bg-color, #1e2021);
			padding: 20px;
			min-width: 300px;
			position: absolute;
			border-radius: 3px;
			box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.2);
			color: var(--color, #fff);
			font-size: 1rem;
			line-height: 1.4;
			z-index: 2;
		}

		.help-tip p:before {
			/* The pointer of the tooltip */
			position: absolute;
			content: '';
			width: 0;
			height: 0;
			border: 6px solid transparent;
			border-bottom-color: var(--bg-color, #1e2021);
			right: 10px;
			top: -12px;
		}

		.help-tip p:after {
			/* Prevents the tooltip from being hidden */
			width: 100%;
			height: 40px;
			content: '';
			position: absolute;
			top: -40px;
			left: 0;
		}

		/* CSS animation */

		@-webkit-keyframes fadeIn {
			0% {
				opacity: 0;
				transform: scale(0.6);
			}

			100% {
				opacity: 100%;
				transform: scale(1);
			}
		}

		@keyframes fadeIn {
			0% {
				opacity: 0;
			}
			100% {
				opacity: 100%;
			}
		}
	`;
}
