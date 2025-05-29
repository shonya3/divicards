import { StashesViewElement } from './e-stashes-view.js';
import { fixture, expect, html } from '@open-wc/testing';
import sinon from 'sinon';
import './e-stashes-view';
import { MockStashLoader } from './data.js';

describe('e-tab-badge-group', async () => {
	let el: StashesViewElement;
	beforeEach(async () => {
		el = await fixture(html`<e-stashes-view .stashLoader=${new MockStashLoader()}></e-stashes-view>`);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});

	it('should emit sample-from-tab', async () => {
		await el.updateComplete;
		const spy = sinon.spy();
		el.addEventListener('sample-from-tab', spy);
		el.stashesButton.click();
		await el.updateComplete;

		const group = el.shadowRoot!.querySelector('e-tab-badge-group')!;
		await group.updateComplete;
		await group.updateComplete;
		const firstTabBadge = group.shadowRoot!.querySelector('e-tab-badge')!;
		firstTabBadge.shadowRoot!.querySelector('input')!.click();
		await el.updateComplete;
		el.getDataButton.click();
		await el.updateComplete;

		expect(spy).to.be.called;
	});
});
