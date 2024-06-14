import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import { createPinia } from 'pinia';

import '@shoelace-style/shoelace/dist/themes/light.css';
import '@shoelace-style/shoelace/dist/themes/dark.css';
import SlAlrt from '@shoelace-style/shoelace/dist/components/alert/alert.component.js';
import SlButton from '@shoelace-style/shoelace/dist/components/button/button.component.js';
import SlIcon from '@shoelace-style/shoelace/dist/components/icon/icon.component.js';
import { addRustListener } from './event';
import { toast } from './toast';
import { handleError } from './error';
import { setBasePath } from '@shoelace-style/shoelace';

setBasePath('/');
SlAlrt.define('sl-alert');
SlButton.define('sl-button');
SlIcon.define('sl-icon');

import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';
import { GoogleAuthElement } from '@divicards/wc/src/wc/google-auth/poe-auth';
import { BasePopupElement } from '@divicards/wc/src/wc/base-popup';
import { ThemeToggle } from '@divicards/wc/src/wc/theme-toggle/theme-toggle';
DropFilesMessageElement.define();
ThemeToggle.define();
PoeAuthElement.define();
GoogleAuthElement.define();
BasePopupElement.define();

const pinia = createPinia();

const app = createApp(App);
app.use(pinia);
app.mount('#app');

window.addEventListener('unhandledrejection', event => handleError(event.reason));
app.config.errorHandler = handleError;
addRustListener('toast', e => {
	toast(e.payload.variant, e.payload.message);
});

declare global {
	interface ObjectConstructor {
		/**
		 * Groups members of an iterable according to the return value of the passed callback.
		 * @param items An iterable.
		 * @param keySelector A callback which will be invoked for each item in items.
		 */
		groupBy<K extends PropertyKey, T>(
			items: Iterable<T>,
			keySelector: (item: T, index: number) => K
		): Partial<Record<K, T[]>>;
	}
}
