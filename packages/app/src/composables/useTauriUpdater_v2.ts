import { Update, check } from '@tauri-apps/plugin-updater';
import { ref, watch } from 'vue';

export function useTauriUpdater() {
	const update = ref<Update | null>(null);
	async function checkUpdate() {
		update.value = await check();
	}

	checkUpdate();

	return {
		update,
	};
}
