import { html, css, svg } from 'lit';
import { BaseElement } from '../../base-element';
import { property } from 'lit/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-fixed-icon': FixedIconElement;
	}
}

export class FixedIconElement extends BaseElement {
	static override tag = 'wc-fixed-icon';
	static override styles = [
		css`
			svg {
				cursor: pointer;
			}
		`,
	];

	@property({ reflect: true, type: Number }) width = 32;
	@property({ reflect: true, type: Number }) height = 32;

	protected override render() {
		return html`<svg width=${this.width} height=${this.height} viewBox="0 0 24 24">
			<path
				fill="currentColor"
				d="M9 2c1.8.6 3 2.3 3 4.2c0 2-1.2 3.7-3 4.3v11c0 .3-.2.5-.5.5h-2c-.3 0-.5-.2-.5-.6v-11c-1.8-.6-3-2.3-3-4.2S4.2 2.6 6 2v3.7h3V2m11.6 11l1.4 1.41L15.47 21L12 17.5l1.4-1.41l2.07 2.08L20.6 13"
			></path>
		</svg>`;
	}
}
