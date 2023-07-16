import { html, css, PropertyValueMap } from 'lit';
import { BaseElement } from './base-element';
import { property } from 'lit-element/decorators.js';
import { Order } from '../../types';
import { classMap } from 'lit/directives/class-map.js';
import { styleMap } from 'lit/directives/style-map.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-order-triangle': OrderTriangleElement;
	}
}

type CssSize = 'px' | 'rem';

const styles = css`
	.order {
		color: var(--color);
		background-color: var(--bg-color);
		width: v-bind(size);
		height: v-bind(size);
		clip-path: polygon(0% 100%, 50% 0%, 100% 100%);
		background-color: var(--color);
		border-radius: 16px;

		transition: 300ms;
		transition-property: background-color, transform, filter;
		filter: brightness(0.8);
		transform: var(--rotation);
		cursor: pointer;
	}

	.order--active {
		filter: brightness(1);
		background-color: cyan;
		transform: var(--rotation);
	}
`;

export class OrderTriangleElement extends BaseElement {
	static htmlTag = 'wc-order-triangle';
	static styles = [styles];

	@property({ reflect: true }) size: `${number}${CssSize}` = '16px';
	@property({ reflect: true }) order: Order = 'asc';
	@property({ type: Boolean }) active = false;

	render() {
		const styles = styleMap({
			width: this.size,
			height: this.size,
			transform: `rotate(${this.order === 'asc' ? '0' : '180'}deg)`,
		});

		const classes = classMap({ 'order--active': this.active, order: true });

		return html`<div style=${styles} class=${classes} title="Order"></div>`;
	}
}
