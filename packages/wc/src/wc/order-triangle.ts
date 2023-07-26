import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { property } from 'lit/decorators.js';
import { classMap } from 'lit/directives/class-map.js';
import { styleMap } from 'lit/directives/style-map.js';
import { Order } from '@divicards/shared/types';

declare global {
	interface HTMLElementTagNameMap {
		'wc-order-triangle': OrderTriangleElement;
	}
}

export type CssSize = 'px' | 'rem';
export type Size = `${number}${CssSize}`;

const degree = (order: Order): number => {
	switch (order) {
		case 'asc':
			return 0;
		case 'desc':
			return 180;
		case 'unordered':
			return 90;
		default:
			throw new Error('invalid order argument');
	}
};

export class OrderTriangleElement extends BaseElement {
	static override tag = 'wc-order-triangle';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) size: Size = '1rem';
	@property({ reflect: true }) order: Order = 'unordered';
	@property({ type: Boolean }) active = false;

	protected override render() {
		const styles = styleMap({
			'--size': this.size,
			transform: `rotate(${degree(this.order)}deg)`,
		});

		const classes = classMap({ 'order--active': this.active, order: true });

		return html`<div style=${styles} class=${classes} title="Order"></div>`;
	}
}

function styles() {
	return css`
		:host {
			display: block;
			height: var(--size, 1rem);
			width: var(--size, 1rem);
		}
		.order {
			color: var(--color);
			height: 100%;
			width: 100%;
			clip-path: polygon(0% 100%, 50% 0%, 100% 100%);
			background-color: var(--color, rgba(255, 255, 255, 0.87));
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
}
