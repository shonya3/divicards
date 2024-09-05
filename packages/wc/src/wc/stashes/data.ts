import { DivinationCardsSample, League } from '@divicards/shared/types';
import { IStashLoader } from '@divicards/shared/IStashLoader';

import stashesData from './json/stashes.json' assert { type: 'json' };
import sampleData from './json/sample.json' assert { type: 'json' };
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';

export const stashes = stashesData as NoItemsTab[];
export const league: League = 'Standard';
export const sample: DivinationCardsSample = sampleData;
import quadStash from './json/QuadStashStd.json';
import fragmentsStash from './json/fragmentsTab.json';
const quad = quadStash as TabWithItems;
const fragments = fragmentsStash as TabWithItems;

let stash: 'quad' | 'fragments' = 'quad';
export class MockStashLoader implements IStashLoader {
	async tab(_tabId: string, _league: string): Promise<TabWithItems> {
		const nextStash = stash === 'quad' ? fragments : quad;
		stash = stash === 'quad' ? 'fragments' : 'quad';
		return nextStash;
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
