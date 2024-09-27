import { fixture, expect, html } from '@open-wc/testing';
import { TabBadgeGroupElement } from './e-tab-badge-group';

describe('wc-tab-badge-group', async () => {
	let el: TabBadgeGroupElement;
	beforeEach(async () => {
		el = await fixture(html`<e-tab-badge-group></e-tab-badge-group>`);
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});
});
