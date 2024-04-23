import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { HelpTipElement } from './help-tip';

declare global {
	interface HTMLElementTagNameMap {
		'wc-drop-files-message': DropFilesMessageElement;
	}
}

/**
 * @summary Message to drop files for main app screen
 */
export class DropFilesMessageElement extends BaseElement {
	static override get defineList() {
		return [HelpTipElement];
	}
	static override tag: string = 'wc-drop-files-message';
	protected override render() {
		return html`<div style="display: flex; gap: 1rem">
			<div class="drop">Drop files <span>Here!</span></div>
			<wc-help-tip>
				<p>Excel, .csv or just .txt</p>
				<p>Required headers: name and amount</p>
				<img src="/simple.png" alt="Example of simple .txt file"
			/></wc-help-tip>
		</div>`;
	}
	static override styles = css`
		:host {
			display: block;
		}
		.drop {
			font-size: 3rem;
		}

		.drop > span {
			color: deeppink;
			font-weight: 700;
		}
	`;
}
