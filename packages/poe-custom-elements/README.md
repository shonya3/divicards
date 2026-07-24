# poe-custom-elements

Path of Exile custom elements


[Demo](https://sparkling-sunburst-2dd5be.netlify.app/)

```bash
npm install poe-custom-elements
```

## Usage

Here is an example with a stash tab

First, you need to open node_modules/poe-custom-elements and copy `poe-ce-assets` dir to your `public` folder.

```js
import 'poe-custom-elements/stash-tab.js';
/* 
 JS object of stash tab from Poe API
 https://www.pathofexile.com/developer/docs/reference#stashes-get
 Check TabWithItems, PoeItem .d.ts types file
(Keep in mind, that exact json importing is a Vite feature, not a real JS) 
*/
import quad from './QuadStashStd.json';

const el = document.createElement('poe-stash-tab');
el.tab = quad;
document.body.append(el);
```

Check [JS Example](https://github.com/shonya3/poe-custom-elements/tree/main/examples/js)

#### Customizing base path for assets
By default, you need to copy assets folder from `node_modules/poe-custom-elements/dist/poe-ce-assets` to your `public` folder.
So that your public folder has `poe-ce-assets` folder. You can customize this path.
```js
import { setBasePath } from 'poe-custom-elements/lib/base_path.js';
setBasePath('/poe');
```
