import { relaunch } from '@tauri-apps/plugin-process';
import { Update, check } from '@tauri-apps/plugin-updater';
import { ref, watch } from 'vue';

export function useTauriUpdater() {
	const update = ref<Update | null>(null);
	async function checkUpdate() {
		update.value = await check();
	}
	async function installAndRelaunch() {
		if (!update.value) {
			return;
		}
		await update.value.downloadAndInstall();
		await relaunch();
	}

	checkUpdate();

	return {
		update,
		installAndRelaunch,
	};
}
