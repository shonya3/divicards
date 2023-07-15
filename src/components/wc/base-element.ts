import { LitElement, css } from 'lit';

const baseStyles = css`
	* {
		padding: 0;
		margin: 0;
		box-sizing: border-box;
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

export class BaseElement extends LitElement {
	static htmlTag: string;
	static baseStyles = baseStyles;
	static define(tag = this.htmlTag) {
		if (!customElements.get(tag)) {
			customElements.define(tag, this);
		}
	}

	emit<T>(eventName: string, detail?: T, options = { bubbles: true, composed: true }) {
		const event = new CustomEvent<T>(eventName, { detail, ...options });
		this.dispatchEvent(event);
	}
}