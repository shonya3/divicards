import { LitElement, css } from 'lit';

export class BaseElement extends LitElement {
	/**
	 * Necessary field to implement for each class. It is used to define the element.
	 */
	static tag: string;
	/**
	 * list of classes of custom elements needed for this element.
	 */
	static defineList: (typeof BaseElement)[] = [];
	static baseStyles = baseStyles();
	static define(tag = this.tag) {
		if (!customElements.get(tag)) {
			if (!this.tag) {
				throw new Error(`${this.name} should have static tag field to define custom element`);
			}
			customElements.define(tag, this);
			for (const ElementClass of this.defineList) {
				ElementClass.define();
			}
		}
	}

	emit<T>(eventName: string, detail?: T, options: EventInit = { bubbles: true, composed: true, cancelable: true }) {
		const event = new CustomEvent<T>(eventName, { detail, ...options });
		this.dispatchEvent(event);
	}
}

function baseStyles() {
	return css`
		:host {
			--bg-color: #242424;
			--color: rgba(255, 255, 255, 0.87);
			--border-color: #646cff;

			font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
			line-height: 24px;
			font-weight: 400;
			color-scheme: dark;
			color: rgba(255, 255, 255, 0.87);
			background-color: #242424;
			font-synthesis: none;
			text-rendering: optimizeLegibility;
			-webkit-font-smoothing: antialiased;
			-moz-osx-font-smoothing: grayscale;
			-webkit-text-size-adjust: 100%;
		}

		a {
			font-weight: 500;
			color: #646cff;
			text-decoration: inherit;
		}
		a:hover {
			color: #535bf2;
		}

		button {
			border-radius: 8px;
			border: 1px solid transparent;
			padding: 0.6em 1.2em;
			font-size: 1em;
			font-weight: 500;
			font-family: inherit;
			background-color: #1a1a1a;
			cursor: pointer;
			transition: border-color 0.25s;
		}
		button:hover {
			border-color: #646cff;
		}
		button:focus,
		button:focus-visible {
			outline: 4px auto -webkit-focus-ring-color;
		}

		input {
			font: inherit;
		}
	`;
}
