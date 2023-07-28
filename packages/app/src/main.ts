import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import { createPinia } from 'pinia';

const pinia = createPinia();

import { listen } from '@tauri-apps/api/event';

const app = createApp(App);
app.use(pinia);
app.mount('#app');
