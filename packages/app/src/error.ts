import { toast } from './toast';
import { TauriError } from '@divicards/shared/error.js';

export const isTauriError = (e: unknown): e is TauriError => {
	if (typeof e === 'object' && e !== null) {
		return Object.hasOwn(e, 'appErrorFromTauri');
	}
	return false;
};

export type SheetsError = {
	error: { code: number; message: string; status: string };
};

export const isSheetsError = (e: unknown): e is SheetsError => {
	if (typeof e === 'object' && e !== null && 'error' in e) {
		return true;
	}
	return false;
};

export const handleError = (err: unknown) => {
	console.log(err);
	if (isTauriError(err)) {
		if (err.kind === 'authError') {
			if (err.authError === 'userDenied') {
				toast('neutral', err.message);
			} else toast('danger', err.message);
		} else toast('danger', err.message);
	} else if (typeof err === 'string') {
		toast('danger', err);
	} else if (err instanceof Error) {
		toast('danger', err.message);
	}

	if (isSheetsError(err)) {
		toast('warning', err.error.message);
	}
};
