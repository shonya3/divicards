import { relaunch } from '@tauri-apps/plugin-process';
import { Update, check } from '@tauri-apps/plugin-updater';
import { shallowRef } from 'vue';

export function useTauriUpdater() {
    const update = shallowRef<Update | null>(null);
    async function checkUpdate() {
        const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;
        if (!isTauri) return;
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
