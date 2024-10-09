// @ts-check
import nodePath from 'node:path';
import { customElementVuejsPlugin } from 'custom-element-vuejs-integration';
/** @import {Plugin} from '@custom-elements-manifest/analyzer' */

/**
 * @type {Array<Plugin>}
 */
const plugins = [
	{
		name: 'my-custom-plugin',
		analyzePhase({ ts, node, moduleDoc, context }) {},
		collectPhase(params) {
			console.log(params.context);
		},
	},
];

export default {
	// watch: true,
	globs: ['src/wc/e-sample-card/e-sample-card.ts'],
	litelement: true,
	plugins: [
		customElementVuejsPlugin({
			outdir: nodePath.join(import.meta.dirname, 'types'),
			fileName: 'web-components.vue.d.ts',
		}),
		...plugins,
	],
};
