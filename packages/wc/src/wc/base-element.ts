import { LitElement, css } from 'lit';

export class BaseElement extends LitElement {
	/** Element tag name */
	static tag: string;
	/**
	 * list of classes of custom elements needed for this element.
	 */
	static defineList: (typeof BaseElement)[] = [];
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
