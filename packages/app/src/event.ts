import { EventCallback, listen } from '@tauri-apps/api/event';
import { ToastVariant } from './toast';

export interface RustEvents {
	'auth-url': {
		type: 'auth-url';
		url: string;
	};
	toast: {
		type: 'toast';
		variant: ToastVariant;
		message: string;
	};
}

export const addRustListener = <EventName extends keyof RustEvents>(
    name: EventName,
    handler: EventCallback<RustEvents[EventName]>
) => {
    const isTauri =
        (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ != null) ||
        (typeof navigator !== 'undefined' && navigator.userAgent.includes('Tauri')) ||
        (typeof import.meta !== 'undefined' && (import.meta as any).env && ((import.meta as any).env.TAURI_PLATFORM ?? (import.meta as any).env.TAURI));
    if (!isTauri) return Promise.resolve(() => {});
    return listen(name, handler);
};
