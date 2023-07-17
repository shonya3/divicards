import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { HelpTipElement } from './help-tip';

declare global {
	interface HTMLElementTagNameMap {
		'wc-drop-files-message': DropFilesMessageElement;
	}
}

const styles = css`
	:host {
		display: block;
	}
	.drop {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.drop > span {
		color: deeppink;
		font-weight: 700;
	}
`;

/**
 * @summary Message to drop files for main app screen
 */
export class DropFilesMessageElement extends BaseElement {
	static define(tag = this.htmlTag) {
		if (!customElements.get(tag)) {
			customElements.define(tag, DropFilesMessageElement);
			HelpTipElement.define();
		}
	}
	static htmlTag: string = 'wc-drop-files-message';
	static styles = styles;
	render() {
		return html`<div style="display: flex; gap: 1rem">
			<div class="drop">Drop files <span>Here!</span></div>
			<wc-help-tip>
				<p>Excel, .csv or just .txt</p>
				<p>Required headers: name and amount</p>
				<img src="/simple.png" alt="Example of simple .txt file"
			/></wc-help-tip>
		</div>`;
	}
}
