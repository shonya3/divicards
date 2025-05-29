import { DivinationCardsSample, League } from '@divicards/shared/types.js';
import { IStashLoader } from '@divicards/shared/IStashLoader.js';

import stashesData from './json/stashes.json' with { type: 'json' };
import sampleData from './json/sample.json' with { type: 'json' };
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';

export const stashes = stashesData as NoItemsTab[];
export const league: League = 'Standard';
export const sample: DivinationCardsSample = sampleData;
import quadStash from './json/QuadStashStd.json'  with { type: 'json' };
import fragmentsStash from './json/fragmentsTab.json'  with { type: 'json' };
const quad = quadStash as TabWithItems;
const fragments = fragmentsStash as TabWithItems;

const sleepSecs = (secs: number): Promise<void> => new Promise(resolve => setTimeout(resolve, secs * 1000));

let stash: 'quad' | 'fragments' = 'quad';
export class MockStashLoader implements IStashLoader {
	async tab(_tabId: string, _league: string): Promise<TabWithItems> {
		const nextStash = stash === 'quad' ? fragments : quad;
		stash = stash === 'quad' ? 'fragments' : 'quad';
		await sleepSecs(0.2);
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
