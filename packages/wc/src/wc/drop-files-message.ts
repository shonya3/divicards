import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { DropFilesTipElement } from './drop-files-tip';

declare global {
	interface HTMLElementTagNameMap {
		'wc-drop-files-message': DropFilesMessageElement;
	}
}

const styles = css`
	.drop {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.drop > span {
		color: deeppink;
		font-weight: 700;
	}
`;

export class DropFilesMessageElement extends BaseElement {
	static define(tag = this.htmlTag) {
		if (!customElements.get(tag)) {
			customElements.define(tag, DropFilesMessageElement);
			DropFilesTipElement.define();
		}
	}
	static htmlTag: string = 'wc-drop-files-message';
	static styles = styles;
	render() {
		return html`<div style="display: flex; gap: 1rem">
			<div class="drop">Drop files <span>Here!</span></div>
			<wc-drop-files-tip></wc-drop-files-tip>
		</div>`;
	}
}
