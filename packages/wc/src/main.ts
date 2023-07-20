import { html, render } from 'lit';
import { stashes } from './wc/stashes/data';
import { TabBadgeGroupElement } from './wc/stashes/tab-badge-group';
import { StashesViewElement } from './wc/stashes/stashes-view';

TabBadgeGroupElement.define();
StashesViewElement.define();

const group = html`<wc-stashes-view .stashes=${stashes} league="Standard"></wc-stashes-view>`;
render(group, document.body);

await new Promise(r => setTimeout(r, 10));
// document.querySelector('wc-tab-badge-group')!.addEventListener('upd:selectedTabs', e => {
// 	console.log(e.detail);
// });
