import { fixture, expect, html } from '@open-wc/testing';
import sinon from 'sinon';
import { TabBadgeElement } from './tab-badge';

describe('<wc-tab-badge>', () => {
	let el: TabBadgeElement;

	beforeEach(async () => {
		TabBadgeElement.define();
		el = await fixture(html`<wc-tab-badge></wc-tab-badge>`);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});

	it('should emit tab-select', async () => {
		const tabSelectSpy = sinon.spy();
		el.addEventListener('tab-select', tabSelectSpy);
		el.tabId = 'Test id';
		el.selected = false;
		el.checkbox.click();
		await el.updateComplete;

		expect(tabSelectSpy).to.have.been.calledOnce;
		expect(tabSelectSpy.args[0][0].detail).to.deep.equal({ tabId: 'Test id', selected: true });
	});
});
