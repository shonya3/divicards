import { LitElement } from 'lit';

export class BaseElement extends LitElement {
	static htmlTag: string;
	static define(tag = this.htmlTag) {
		if (!customElements.get(tag)) {
			customElements.define(tag, this);
		}
	}

	emit<T>(eventName: string, detail: T, options = { bubbles: true, composed: true }) {
		const event = new CustomEvent<T>(eventName, { detail, ...options });
		this.dispatchEvent(event);
	}
}
