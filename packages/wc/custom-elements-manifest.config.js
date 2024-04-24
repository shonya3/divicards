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

						// const classDoc = moduleDoc?.declarations?.find(declaration => declaration.name === className);
						// console.log(classDoc);

						// const importPath = moduleDoc.path;

						// // This is kind of a best guess at components. "thing.component.ts"
						// if (!importPath.endsWith('.component.ts')) {
						// 	return;
						// }

						// const tagNameWithoutPrefix = path.basename(importPath, '.component.ts');
						// const tagName = 'sl-' + tagNameWithoutPrefix;

						// classDoc.tagNameWithoutPrefix = tagNameWithoutPrefix;
						// classDoc.tagName = tagName;

						// // This used to be set to true by @customElement
						// classDoc.customElement = true;
					}
				}
			},
		},

		customElementVuejsPlugin({
			outdir: './types',
			fileName: 'vue.d.ts',
			// componentTypePath(name, tag) {
			// 	console.log({ name, tag });
			// },
			// componentTypePath: (name, tag) => {
			// 	console.log({ name, tag });
			// 	return './src/e-button.ts';
			// },
			// globalTypePath: './src/appTypes.ts',
		}),
	],
};
