import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import { createPinia } from 'pinia';

import '@shoelace-style/shoelace/dist/themes/dark.css';
import SlAlrt from '@shoelace-style/shoelace/dist/components/alert/alert.component.js';
import SlButton from '@shoelace-style/shoelace/dist/components/button/button.component.js';
import SlIcon from '@shoelace-style/shoelace/dist/components/icon/icon.component.js';
import { addRustListener } from './event';
import { toast } from './toast';

SlAlrt.define('sl-alert');
SlButton.define('sl-button');
SlIcon.define('sl-icon');

const pinia = createPinia();

const app = createApp(App);
app.use(pinia);
app.mount('#app');

app.config.errorHandler = err => {
	console.log('from Vue error handler', err);
	if (typeof err === 'string') {
		toast('danger', err);
	} else if (err instanceof Error) {
		toast('danger', err.message);
	}
};

addRustListener('toast', e => {
	toast(e.payload.variant, e.payload.message);
});
