import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import './poe-item.js';
import { basePath } from '../lib/base_path.js';
import { cardElementData } from './divination-card/data.js';

/**
 * Itemized divination card with divination card element on hover, that requires only a name of card.
 * @summary Itemized divination card with divination card element on hover,
 *          that requires only a name of card.
 *
 * @cssprop --poe-item-size        - Size of item.
 * @cssprop --stack-size-font-size - Font size of stack size.
 *
 * Copy item-card and divination-card folders to your public dir.
 */
@customElement('poe-item-card')
export class PoeItemCardElement extends LitElement {
	@property({ reflect: true }) name = 'Rain of Chaos';
	@property({ type: Number, attribute: 'stack-size' }) stackSize?: number;

	protected render(): TemplateResult {
		return html`<poe-item
			.item=${{
				baseType: this.name,
				stackSize: this.stackSize,
				maxStackSize: cardElementData.find(card => card.name === this.name)?.stackSize ?? undefined,
				icon: `${basePath()}/item-card/InventoryIcon.png`,
				w: 1,
				h: 1,
				identified: true,
				frameType: 6,
				league: 'Standard',
				inventoryId: '',
				name: '',
				x: 0,
				y: 0,
				id: '',
				verified: false,
				typeLine: this.name,
				ilvl: 0,
			}}
		></poe-item>`;
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			display: inline-block;
			width: fit-content;
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-item-card': PoeItemCardElement;
	}
}
