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
