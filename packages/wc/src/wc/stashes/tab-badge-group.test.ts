import { fixture, expect, html } from '@open-wc/testing';
import sinon from 'sinon';
import { TabBadgeGroupElement } from './tab-badge-group';

describe('wc-tab-badge-group', async () => {
	let el: TabBadgeGroupElement;
	beforeEach(async () => {
		TabBadgeGroupElement.define();
		el = await fixture(html`<wc-tab-badge-group></wc-tab-badge-group>`);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});
});
