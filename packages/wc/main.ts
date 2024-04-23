// import { LeagueSelectElement } from './src/wc/league-select';
// import { SampleCardElement } from './src/wc/sample-card/sample-card';

import { FormExportSampleElement } from './src/wc/form-export-sample/form-export-sample';
import { LeagueSelectElement } from './src/wc/league-select';
import { OrderTriangleElement } from './src/wc/order-triangle';
import { PoeAuthElement } from './src/wc/poe-auth';

// SampleCardElement.define();

// const el = document.createElement('wc-sample-card');
// document.body.append(el);

// el.addEventListener('upd:league', e => {
// 	console.log(e);
// });

// console.log(el.league);

FormExportSampleElement.define();
document.body.append(document.createElement('wc-form-export-sample'));
