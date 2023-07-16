import { html, css } from 'lit';
import { BaseElement } from './base-element';
import { property, state } from 'lit-element/decorators.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-test-component': TestComponent;
	}
}

export class TestComponent extends BaseElement {
	static htmlTag = 'wc-test-component';
	static styles = css``;

	@property({ type: Number }) counter = 0;

	@property({ type: Number, reflect: true }) perPage = 50;
	@property({ type: Number, reflect: true }) page = 1;
	@property() nameQuery = '';

	// get filtered() {
	// 	return this.stashes;
	// }

	get shouldBeVisible() {
		return this.counter < 30;
	}

	render() {
		return html`<div>${this.shouldBeVisible ? this.counter : 'Nothing'}</div> `;
	}

	firstUpdated() {
		console.log('hi');
		setInterval(() => {
			this.counter++;
		}, 300);
	}
}
