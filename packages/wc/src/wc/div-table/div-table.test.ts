import { fixture, expect, html } from '@open-wc/testing';
import { DivTableElement } from './div-table';
import { Order } from '@divicards/shared/types';
import { Column } from './types';
import sinon from 'sinon';

import { sendMouse } from '@web/test-runner-commands';

function determineMousePosition(el: Element, position: string, offsetX: number, offsetY: number) {
	const { x, y, width, height } = el.getBoundingClientRect();
	const centerX = Math.floor(x + window.pageXOffset + width / 2);
	const centerY = Math.floor(y + window.pageYOffset + height / 2);
	let clickX: number;
	let clickY: number;

	switch (position) {
		case 'top':
			clickX = centerX;
			clickY = y;
			break;
		case 'right':
			clickX = x + width - 1;
			clickY = centerY;
			break;
		case 'bottom':
			clickX = centerX;
			clickY = y + height - 1;
			break;
		case 'left':
			clickX = x;
			clickY = centerY;
			break;
		default:
			clickX = centerX;
			clickY = centerY;
	}

	clickX += offsetX;
	clickY += offsetY;
	return { clickX, clickY };
}

/** A testing utility that measures an element's position and clicks on it. */
export async function clickOnElement(
	/** The element to click */
	el: Element,
	/** The location of the element to click */
	position: 'top' | 'right' | 'bottom' | 'left' | 'center' = 'center',
	/** The horizontal offset to apply to the position when clicking */
	offsetX = 0,
	/** The vertical offset to apply to the position when clicking */
	offsetY = 0
) {
	const { clickX, clickY } = determineMousePosition(el, position, offsetX, offsetY);

	await sendMouse({ type: 'click', position: [clickX, clickY] });
}

describe('<wc-div-table>', () => {
	let el: DivTableElement;

	beforeEach(async () => {
		DivTableElement.define();
		el = await fixture(html` <wc-div-table> </wc-div-table> `);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});

	it('emits column-order-changed', async () => {
		const selectedChangeSpy = sinon.spy();
		el.addEventListener('column-order-changed', selectedChangeSpy);

		const node = el.shadowRoot!.querySelector('wc-order-triangle')!;
		node.click();
		await el.updateComplete;
		console.log(node);
		expect(selectedChangeSpy).to.have.been.called;
	});
});