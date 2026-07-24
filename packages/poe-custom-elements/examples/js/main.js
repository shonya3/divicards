import 'poe-custom-elements/stash-tab.js';
/* 
 JS object of stash tab from Poe API
 https://www.pathofexile.com/developer/docs/reference#stashes-get
 Check TabWithItems, PoeItem .d.ts types file 
*/
import quad from './QuadStashStd.json';

const el = document.createElement('poe-stash-tab');
el.tab = quad;
document.body.append(el);
