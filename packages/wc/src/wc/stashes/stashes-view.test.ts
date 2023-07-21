import { StashesViewElement } from './stashes-view';
import { fixture, expect, html } from '@open-wc/testing';
import sinon from 'sinon';
import { MockStashLoader } from './data';

describe('<wc-tab-badge-group>', async () => {
	let el: StashesViewElement;
	beforeEach(async () => {
		StashesViewElement.define();
		el = await fixture(html`<wc-stashes-view .stashLoader=${new MockStashLoader()}></wc-stashes-view>`);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});

	it('should emit tab-data', async () => {
		await el.updateComplete;
		const spy = sinon.spy();
		el.addEventListener('tab-data', spy);
		el.stashesButton.click();
		await el.updateComplete;

		const group = el.shadowRoot!.querySelector('wc-tab-badge-group')!;
		await group.updateComplete;
		await group.updateComplete;
		const firstTabBadge = group.shadowRoot!.querySelector('wc-tab-badge')!;
		firstTabBadge.shadowRoot!.querySelector('input')!.click();
		await el.updateComplete;
		el.getDataButton.click();
		await el.updateComplete;

		expect(spy).to.be.called;
	});
});
