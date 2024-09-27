import { DivinationCardsSample, League } from '@divicards/shared/types';
import { To } from '@divicards/wc/src/wc/e-sample-card/e-form-export-sample/e-form-export-sample';
import { defineStore } from 'pinia';

export interface ExportSampleState {
	sample: DivinationCardsSample | null;
	league: League | null;
	sheetsError: string | null;
	to: To;
	filename: string;
}

export const useExportSampleStore = defineStore('exportSample', {
	state: (): ExportSampleState => ({
		sample: null,
		league: null,
		sheetsError: null,
		to: 'file',
		filename: 'sample.csv',
	}),
});
