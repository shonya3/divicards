export type StashTabError = {
	stashId: string;
	league: string;
	appErrorFromTauri: true;
	kind: 'stashTabError';
	message: string;
};
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
	  }
	| StashTabError;

export function isStashTabError(err: unknown): err is StashTabError {
	return typeof err === 'object' && err != null && 'kind' in err && err.kind === 'stashTabError';
}
