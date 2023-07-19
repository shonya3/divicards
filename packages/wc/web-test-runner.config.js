import { esbuildPlugin } from '@web/dev-server-esbuild';
import { globbySync } from 'globby';
import { playwrightLauncher } from '@web/test-runner-playwright';

export default {
	rootDir: '.',
	files: 'src/**/*.test.ts', // "default" group
	concurrentBrowsers: 3,
	nodeResolve: true,
	testFramework: {
		config: {
			timeout: 3000,
			retries: 1,
		},
	},
	plugins: [
		esbuildPlugin({
			ts: true,
			target: 'es2020',
		}),
	],
	browsers: [
		playwrightLauncher({ product: 'chromium' }),
		playwrightLauncher({ product: 'firefox' }),
		playwrightLauncher({ product: 'webkit' }),
	],

	// Create a named group for every test file to enable running single tests. If a test file is `split-panel.test.ts`
	// then you can run `npm run test -- --group split-panel` to run only that component's tests.
	groups: globbySync('src/**/*.test.ts').map(path => {
		const groupName = path.match(/^.*\/(?<fileName>.*)\.test\.ts/).groups.fileName;
		return {
			name: groupName,
			files: path,
		};
	}),
};
