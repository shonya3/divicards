import { toast } from './toast';

export type TauriError =
	| {
			appErrorFromTauri: true;
			kind: 'httpError' | 'serdeError' | 'diviError';
			message: string;
	  }
	| {
			appErrorFromTauri: true;
			kind: 'authError';
			authError: 'userDenied' | 'otherWithDescription' | 'failed';
			message: string;
	  };

export const isTauriError = (e: unknown): e is TauriError => {
	if (typeof e === 'object' && e !== null) {
		return Object.hasOwn(e, 'appErrorFromTauri');
	}
	return false;
};

export const handleError = (err: unknown) => {
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
};
