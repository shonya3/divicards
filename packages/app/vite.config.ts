import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import VueDevTools from 'vite-plugin-vue-devtools';
import { viteStaticCopy } from 'vite-plugin-static-copy';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		vue({
			script: {
				defineModel: true,
			},
			template: {
				compilerOptions: {
					isCustomElement(tag: string) {
						return tag.includes('-');
					},
				},
			},
		}),
		VueDevTools(),
		viteStaticCopy({
			targets: [
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/grip-vertical.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/question-circle.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/info-circle.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/check2-circle.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/gear.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/exclamation-triangle.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/exclamation-octagon.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/filetype-csv.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/file-earmark-spreadsheet.svg',
					dest: 'assets/icons',
				},
				{
					src: 'node_modules/@shoelace-style/shoelace/dist/assets/icons/x-lg.svg',
					dest: 'assets/icons',
				},
			],
		}),
	],
});
