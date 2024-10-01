import { expect, html, fixture } from '@open-wc/testing';
import sinon from 'sinon';
import { SampleCardElement } from './e-sample-card';
import { props } from './data';
import { sendKeys } from '@web/test-runner-commands';
import './e-sample-card';
import type { Props } from './e-sample-card';

describe('<wc-sample-card>', () => {
	let el: SampleCardElement;

	beforeEach(async () => {
		el = await fixture(html`<e-sample-card></e-sample-card>`);
		Object.assign(el, { ...props });
	});

	it('should render a component', () => {
		expect(el).to.exist;
	});

	it('emits upd:selected on checkbox', async () => {
		await el.updateComplete;
		const checkboxSpy = sinon.spy();
		el.addEventListener('upd:selected', checkboxSpy);
		el.selectedCheckbox.click();
		await el.updateComplete;

		expect(checkboxSpy).to.be.calledOnce;
	});

	it('emits change:league on selecting league', async () => {
		await el.updateComplete;
		const spy = sinon.spy();
		el.addEventListener('change:league', spy);

		el.leagueSelect.focus();
		await sendKeys({ press: 'ArrowDown' });
		await el.updateComplete;

		const event = spy.args[0][0];
		expect(spy).to.be.called;
		expect(el.leagueSelect.league).to.be.equal(el.league);
		expect(event.detail).to.be.equal(el.league);
	});

	it('emits delete on btn-delete click', async () => {
		await el.updateComplete;
		const spy = sinon.spy();
		el.addEventListener('delete', spy);

		const button = el.shadowRoot!.querySelector('#btn-delete') as HTMLButtonElement;
		expect(button).to.exist;

		button.click();

		expect(spy).to.be.called;
		const event = spy.args[0][0];
		expect(event.detail).to.not.equal('NO ID');
		expect(event.detail).to.be.equal(el.uuid);
	});

	it('emits upd:minimumCardPrice on slider', async () => {
		await el.updateComplete;
		const spy = sinon.spy();
		el.addEventListener('upd:minimumCardPrice', spy);
		el.priceSlider.focus();
		await sendKeys({ press: 'ArrowRight' });
		await el.updateComplete;

		const event = spy.args[0][0];
		expect(spy).to.be.calledOnce;
		expect(event.detail).to.be.equal(el.minimumCardPrice);
	});

	it('table cards get updated when filecard cards get updated', async () => {
		await el.updateComplete;
		const newProps: Props = {
			league: 'Standard',
			filename: 'Standard.csv',
			selected: false,
			uuid: '2',
			minimumCardPrice: 0,
			sample: {
				cards: [{ name: 'Rain of Chaos', price: 1, amount: 1, sum: 1, weight: 0 }],
				notCards: [],
				fixedNames: [],
			},
		};

		Object.assign(el, { ...newProps });

		await el.updateComplete;
		expect(el.table.cards.length).to.be.equal(1);
		expect(el.table.cards[0].name).to.be.equal('Rain of Chaos');
		expect(el.table.filteredRecords.length).to.be.equal(1);
		expect(el.table.filteredRecords[0].name).to.be.equal('Rain of Chaos');
	});
});
