import { fixture, expect, html } from '@open-wc/testing';
import { DivTableElement } from './div-table';
import sinon from 'sinon';

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
