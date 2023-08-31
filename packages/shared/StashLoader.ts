import { command } from '../app/src/command';
import { NoItemsTab } from './poe.types';
import { DivinationCardsSample, League } from './types';

export interface IStashLoader {
	tabs(league: League): Promise<NoItemsTab[]>;
	sampleFromTab(tabId: string, league: League): Promise<DivinationCardsSample>;
}

export class StashLoader implements IStashLoader {
	sampleFromTab(tabId: string, league: League): Promise<DivinationCardsSample> {
		return command('sample_from_tab', { league, stashId: tabId });
	}

	async tabs(league: League): Promise<NoItemsTab[]> {
		const { stashes = [] } = await command('stashes', { league });
		return this.#flattenStashes(stashes);
	}

	#flattenStashes(tabs: NoItemsTab[]): NoItemsTab[] {
		const flat: NoItemsTab[] = [];

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
