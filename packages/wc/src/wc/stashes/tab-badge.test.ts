import { fixture, expect, html } from '@open-wc/testing';
import sinon from 'sinon';
import { TabBadgeElement } from './e-tab-badge';
import './e-tab-badge';

describe('e-tab-badge', () => {
	let el: TabBadgeElement;

	beforeEach(async () => {
		el = await fixture(html`<e-tab-badge></e-tab-badge>`);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});

	it('should emit tab-select', async () => {
		const tabSelectSpy = sinon.spy();
		el.addEventListener('tab-select', tabSelectSpy);
		el.selected = false;
		el.checkbox.click();
		await el.updateComplete;

		expect(tabSelectSpy).to.have.been.calledOnce;
		expect(tabSelectSpy.args[0][0].detail).to.deep.equal({ tabId: 'Test id', selected: true, name: 'Heist' });
	});
});
