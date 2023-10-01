import { DivinationCardsSample, League } from '@divicards/shared/types';
import { To } from '@divicards/wc/src/wc/form-export-sample/form-export-sample';
import { ref } from 'vue';

export const useExportState = () => {
	const sample = ref<DivinationCardsSample | null>(null);
	const league = ref<League | null>(null);
	const sheetsError = ref<string | null>(null);
	const to = ref<To>('sheets');
	const filename = ref<string>('file.csv');

	return {
		sample,
		league,
		sheetsError,
		to,
		filename,
	};
};
