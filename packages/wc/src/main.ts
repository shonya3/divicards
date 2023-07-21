import { html, render } from 'lit';
import { MockStashLoader, stashes } from './wc/stashes/data';
import { TabBadgeGroupElement } from './wc/stashes/tab-badge-group';
import { StashesViewElement } from './wc/stashes/stashes-view';

TabBadgeGroupElement.define();
StashesViewElement.define();

const stashesView = document.createElement('wc-stashes-view');
document.body.append(stashesView);

stashesView.stashLoader = new MockStashLoader();
await stashesView.updateComplete;

// console.log(stashesView.stashesButton);
// stashesView.stashesButton.click();
// console.log(stashesView.getDataButton);
