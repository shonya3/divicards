// @ts-check
import nodePath from 'node:path';
import { customElementVuejsPlugin } from 'custom-element-vuejs-integration';

export default {
	watch: true,
	litelement: true,
	plugins: [
		{
			name: 'shoelace-infer-tag-names',
			analyzePhase({ ts, node, moduleDoc }) {
				switch (node.kind) {
					case ts.SyntaxKind.ClassDeclaration: {
						// console.log(node.name.getText());
						const className = node.name.getText();
						const basename = nodePath.basename(moduleDoc.path);
						const name = basename.slice(0, basename.indexOf('.'));
						if (name.includes('-')) {
							const classDoc = moduleDoc?.declarations?.find(
								declaration => declaration.name === className
							);
							classDoc.tagName = `wc-${name}`;
							classDoc.customElement = true;
						}
					}
				}
			},
		},

		customElementVuejsPlugin({
			outdir: './types',
			fileName: 'web-components.vue.d.ts',
		}),
	],
};
