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
	return listen(name, handler);
};
