import { BasePopupElement } from './wc/base-popup';
import { DivTableElement } from './wc/div-table/div-table';
import { cards } from './wc/div-table/div-table.props';
DivTableElement.define();
BasePopupElement.define();

const table = document.querySelector('wc-div-table')!;
table.cards = cards;

const popup = document.querySelector('wc-base-popup')!;
const button = document.querySelector('button');

await popup.updateComplete;
popup.dialog.showModal();
button?.addEventListener('click', () => {
	popup.dialog.showModal();
});
