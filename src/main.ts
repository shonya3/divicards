import { createApp } from 'vue';
import './style.css';
import App from './App.vue';

createApp(App).mount('#app');

import { invoke } from '@tauri-apps/api';

invoke('get_hashmap').then(r => {
	console.log(r);
});

// import { InvokeArgs } from '@tauri-apps/api/tauri';
// const r = command('read_polish_csv', {
// 	csvString: `stackSize,,name,calculated,total
// 49,The Opulent,1.0,49.0
// 0,The Price of Devotion,5772.65,0.0
// 0,Perfection,3.0,0.0
// 65,Three Faces in the Dark,1.0,65.0`,
// });
