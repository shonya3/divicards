import { fixture, expect, html } from '@open-wc/testing';
import { SampleTableElement } from './e-sample-table.js';
import sinon from 'sinon';
import './e-sample-table';

describe('<e-sample-table>', () => {
	let el: SampleTableElement;

	beforeEach(async () => {
		el = await fixture(html` <e-sample-table> </e-sample-table> `);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});

	it('emits column-order-changed', async () => {
		const selectedChangeSpy = sinon.spy();
		el.addEventListener('column-order-changed', selectedChangeSpy);

		const node = el.shadowRoot!.querySelector('e-order-triangle')!;
		node.click();
		await el.updateComplete;
		console.log(node);
		expect(selectedChangeSpy).to.have.been.called;
	});
});
