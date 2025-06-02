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
				<div class="tooltip-box">
					<slot></slot>
				</div>
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
		}

		.tooltip-box {
			text-align: left;
			background-color: var(--e-help-tip-bg, var(--bg-color, var(--sl-color-neutral-900)));
			padding: 1rem;
			min-width: 300px;
			border-radius: 8px;
			border: 1px solid
				var(
					--e-help-tip-border-color,
					color-mix(in srgb, var(--color, var(--sl-color-neutral-0)) 20%, transparent)
				);
			box-shadow: var(
				--e-help-tip-shadow,
				0 2px 12px color-mix(in srgb, var(--color, var(--sl-color-neutral-0)) 10%, transparent)
			);
			color: var(--e-help-tip-text-color, var(--color, var(--sl-color-neutral-0)));
			font-size: 1rem;
			line-height: 1.4;
		}
	`;
}
