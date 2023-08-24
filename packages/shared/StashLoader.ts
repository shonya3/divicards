import { command } from '../app/src/command';
import { StashTab } from './poe.types';
import { DivinationCardsSample, League, Result } from './types';

export interface IStashLoader {
	tabs(league: League): Promise<StashTab[]>;
	sampleFromTab(tabId: StashTab['id'], league: League): Promise<Result<DivinationCardsSample>>;
}

export class StashLoader implements IStashLoader {
	sampleFromTab(tabId: string, league: League): Promise<Result<DivinationCardsSample>> {
		return command('sample_from_tab', { league, stashId: tabId });
	}

	async tabs(league: League): Promise<StashTab[]> {
		const { stashes = [] } = await command('stashes', { league });
		return this.#flattenStashes(stashes);
	}

	#flattenStashes(tabs: StashTab[]): StashTab[] {
		const flat: StashTab[] = [];

		for (const tab of tabs) {
			if (tab.type !== 'Folder') {
				flat.push(tab);
			}

			if (tab.children) {
				for (const childTab of tab.children) {
					flat.push(childTab);
				}
			}
		}

		return flat;
	}
}
