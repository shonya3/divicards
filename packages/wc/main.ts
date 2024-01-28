import { LeagueSelectElement } from './src/wc/league-select';
import { SampleCardElement } from './src/wc/sample-card/sample-card';

SampleCardElement.define();

const el = document.createElement('wc-sample-card');
document.body.append(el);

el.addEventListener('upd:league', e => {
	console.log(e);
});

console.log(el.league);
