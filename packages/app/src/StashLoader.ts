import { IStashLoader } from '@divicards/shared/IStashLoader';
import { DivinationCardsSample, League } from '@divicards/shared/types';
import { command } from './command';
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';

export class StashLoader implements IStashLoader {
	tab(tabId: string, league: string): Promise<TabWithItems> {
		return command('tab_with_items', { league, stashId: tabId });
	}

	sampleFromTab(tabId: string, league: League): Promise<DivinationCardsSample> {
		return command('sample_from_tab', { league, stashId: tabId });
	}

	async tabs(league: League | string): Promise<NoItemsTab[]> {
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
