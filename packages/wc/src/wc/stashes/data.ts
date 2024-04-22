import { DivinationCardsSample, League } from '@divicards/shared/types';
import { NoItemsTab, TabWithItems } from '@divicards/shared/poe.types';
import { IStashLoader } from '@divicards/shared/IStashLoader';

import stashesData from './json/stashes.json' assert { type: 'json' };
import sampleData from './json/sample.json' assert { type: 'json' };

export const stashes: NoItemsTab[] = stashesData;
export const league: League = 'Standard';
export const sample: DivinationCardsSample = sampleData;

export class MockStashLoader implements IStashLoader {
	tab(_tabId: string, _league: string): Promise<TabWithItems> {
		return {} as any;
	}
	sampleFromTab(_tabId: string, _league: League): Promise<DivinationCardsSample> {
		return new Promise(r =>
			setTimeout(() => {
				r(sample);
			}, 50)
		);
	}
	tabs(_league: League): Promise<NoItemsTab[]> {
		return new Promise(r => r(stashes));
	}
}
