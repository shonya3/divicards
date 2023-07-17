import { html } from 'lit';
import { BaseElement } from './base-element';
import { HelpTipElement } from './help-tip';

declare global {
	interface HTMLElementTagNameMap {
		'wc-drop-files-tip': DropFilesTipElement;
	}
}

export class DropFilesTipElement extends BaseElement {
	static define(tag = 'wc-drop-files-tip') {
		if (!customElements.get(tag)) {
			customElements.define(tag, DropFilesTipElement);
			HelpTipElement.define();
		}
	}
	static htmlTag: string = 'drop-files-tip';
	render() {
		return html`<wc-help-tip>
			<p>Excel, .csv or just .txt</p>
			<p>Required headers: name and amount</p>
			<img src="/simple.png" alt=""
		/></wc-help-tip>`;
	}
}
