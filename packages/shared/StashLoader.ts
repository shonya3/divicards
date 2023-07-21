import { command } from '../app/src/command';
import { StashTab } from './poe.types';
import { League } from './types';

export interface IStashLoader {
	tab(tabId: StashTab['id'], league: League): Promise<StashTab>;
	tabs(league: League): Promise<StashTab[]>;
}

export class StashLoader implements IStashLoader {
	async tab(tabId: StashTab['id'], league: League): Promise<StashTab> {
		const { stash: tab } = await command('stash', { league, stashId: tabId });
		return tab;
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
