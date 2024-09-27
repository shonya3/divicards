// import { LeagueSelectElement } from './src/wc/league-select';
// import { SampleCardElement } from './src/wc/sample-card/sample-card';

import { FormExportSampleElement } from './src/wc/form-export-sample/form-export-sample';
import { LeagueSelectElement } from './src/wc/league-select';
import { OrderTriangleElement } from './src/wc/e-order-triangle';
import { PoeAuthElement } from './src/wc/poe-auth';
import { MockStashLoader, stashes } from './src/wc/stashes/data';
import { StashesViewElement } from './src/wc/stashes/stashes-view';

// StashesViewElement.define();
// const stashesView = document.createElement('wc-stashes-view');
// stashesView.stashLoader = new MockStashLoader();
// document.body.append(stashesView);
// await stashesView.updateComplete;
// stashesView.stashes = stashes;

FormExportSampleElement.define();
const form = document.createElement('wc-form-export-sample');
document.body.append(form);
form.addEventListener('submit', e => {
	console.log(e);
});
