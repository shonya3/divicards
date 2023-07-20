// import { BasePopupElement } from './wc/base-popup';
// import { DivTableElement } from './wc/div-table/div-table';
// import { cards } from './wc/div-table/div-table.props';
// DivTableElement.define();
// BasePopupElement.define();

import { fileCardProps } from './wc/file-card/data';
import { FileCardElement } from './wc/file-card/file-card';

// const table = document.querySelector('wc-div-table')!;
// table.cards = cards;

// const popup = document.querySelector('wc-base-popup')!;
// const button = document.querySelector('button');

// await popup.updateComplete;
// popup.dialog.showModal();
// button?.addEventListener('click', () => {
// 	popup.dialog.showModal();
// });

FileCardElement.define();

const fileCard = document.querySelector('wc-file-card')!;
Object.assign(fileCard, { ...fileCardProps });
fileCard.league = 'Hardcore';
await fileCard.updateComplete;
document.addEventListener('update:selected', e => {
	console.log(e);
});

fileCard.selectedCheckbox.click();
fileCard.addEventListener('league-change', e => {
	console.log('league-change', e);
});

fileCard.addEventListener('update:minimumCardPrice', e => {
	console.log('update:minimumCardPrice', e);
});

setTimeout(() => {
	fileCard.leagueSelect.focus();
}, 500);
