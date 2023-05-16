import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import { createPinia } from 'pinia';
import { command } from './command';

createApp(App).use(createPinia()).mount('#app');

const google = await command('google_auth', {});
console.log({ google });
