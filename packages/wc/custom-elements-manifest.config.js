// @ts-check
import nodePath from 'node:path';
import { customElementVuejsPlugin } from 'custom-element-vuejs-integration';

export default {
	watch: true,
	litelement: true,
	plugins: [
		customElementVuejsPlugin({
			outdir: nodePath.join(import.meta.dirname, 'types'),
			fileName: 'web-components.vue.d.ts',
		}),
	],
};
