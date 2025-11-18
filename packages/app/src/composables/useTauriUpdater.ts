import { relaunch } from '@tauri-apps/plugin-process';
import { Update, check } from '@tauri-apps/plugin-updater';
import { shallowRef } from 'vue';

export function useTauriUpdater() {
    const update = shallowRef<Update | null>(null);
    async function checkUpdate() {
        const isTauri =
            (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ != null) ||
            (typeof navigator !== 'undefined' && navigator.userAgent.includes('Tauri')) ||
            (typeof import.meta !== 'undefined' && (import.meta as any).env && ((import.meta as any).env.TAURI_PLATFORM ?? (import.meta as any).env.TAURI));
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
