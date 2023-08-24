import { DivinationCardsSample, League, Result } from '@divicards/shared/types';
import { StashTab } from '@divicards/shared/poe.types';
import { IStashLoader } from '@divicards/shared/StashLoader';

import stashData from './stash.json' assert { type: 'json' };
import stashesData from './stashes.json' assert { type: 'json' };
import resultSample from './resultSample.json' assert { type: 'json' };

export const stash: StashTab = stashData as unknown as StashTab;
export const stashes: StashTab[] = stashesData;
export const league: League = 'Standard';
export const sample: Result<DivinationCardsSample> = resultSample as Result<DivinationCardsSample>;

export class MockStashLoader implements IStashLoader {
	sampleFromTab(_tabId: string, _league: League): Promise<Result<DivinationCardsSample>> {
		return new Promise(r => r(sample));
	}
	tabs(_league: League): Promise<StashTab[]> {
		return new Promise(r => r(stashes));
	}
}
