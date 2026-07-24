import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';

@customElement('tooltip-json-icon')
export class JsonIconElement extends LitElement {
	@property({ type: Boolean, reflect: true }) showing = false;

	showWithAutohide(): void {
		this.showing = true;
		setTimeout(() => {
			this.showing = false;
		}, 2000);
	}

	protected render(): TemplateResult {
		return html`
			<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 32 32">
				<path
					fill="#d6d024"
					d="M4 20v2h4.586L2 28.586L3.414 30L10 23.414V28h2v-8zm25-8l-2-6h-2v10h2v-6l2 6h2V6h-2zm-7.666-6h-2.667A1.67 1.67 0 0 0 17 7.667v6.667A1.67 1.67 0 0 0 18.666 16h2.667A1.67 1.67 0 0 0 23 14.334V7.667A1.67 1.67 0 0 0 21.334 6M21 14h-2V8h2zM9 7.667V10a2 2 0 0 0 2 2h2v2H9v2h4.334A1.67 1.67 0 0 0 15 14.334V12a2 2 0 0 0-2-2h-2V8h4V6h-4.334A1.67 1.67 0 0 0 9 7.667M5 14H3v-2H1v2.334A1.67 1.67 0 0 0 2.667 16h2.667A1.67 1.67 0 0 0 7 14.334V6H5z"
				/>
			</svg>
		`;
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			position: absolute;
			top: 100px;
			right: 0px;
			color: yellow;
			opacity: 0;
			transform: scale(0.75);
			z-index: 20;
			pointer-events: none;
			transition: opacity, transform, top;
			transition-duration: 0.2s;
		}

		:host([showing]) {
			opacity: 1;
			transform: scale(1);
			top: 0px;
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'tooltip-json-icon': JsonIconElement;
	}
}
