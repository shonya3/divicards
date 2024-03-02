import { UpdateManifest, UpdateResult, checkUpdate as tauriCheckUpdate, installUpdate } from '@tauri-apps/api/updater';
import { computed, ref } from 'vue';

export function useTauriUpdater() {
	const updateResult = ref<UpdateResult | null>(null);
	const manifest = computed<UpdateManifest | null>(() => updateResult.value?.manifest ?? null);
	const shouldUpdate = computed<boolean>(() => updateResult.value?.shouldUpdate ?? false);

	async function checkUpdate() {
		try {
			if (import.meta.env.DEV) {
				updateResult.value = await mockCheckUpdate();
				// updateResult.value = await tauriCheckUpdate();
			} else {
				updateResult.value = await tauriCheckUpdate();
			}
		} catch (err) {
			console.warn('useTauriUpdater: checkUpdate Error', err);
		}
	}

	checkUpdate();

	return {
		manifest,
		shouldUpdate,
		checkUpdate,
		installUpdate,
	};
}

export async function mockCheckUpdate(): Promise<UpdateResult> {
	return {
		manifest: {
			version: '0.5.5',
			date: '2024-03-01 9:10:51.791 +00:00:00',
			body: 'Release notes',
		},
		shouldUpdate: true,
	};
}
