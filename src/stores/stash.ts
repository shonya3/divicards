import { defineStore } from 'pinia';
import { StashTab } from '../poe/types';
import { League } from '../types';
import { ACTIVE_LEAGUE } from '../lib';
import { stashes, stash } from '../poe/api';
import { flatten, toStateful } from '../poe/stash';

export type StatefulStashTab = StashTab & { selected: boolean };

export const useStashStore = defineStore('stashStore', {
	state: (): {
		stashes: StatefulStashTab[];
	} => ({
		stashes: [],
	}),
	getters: {
		selectedTabs(): StatefulStashTab[] {
			return this.stashes.filter(stash => stash.selected === true);
		},

		selectedTabsIds(): string[] {
			return this.selectedTabs.map(tab => tab.id);
		},
	},
	actions: {
		async fetchStashes(league: League = ACTIVE_LEAGUE) {
			const tabs = await stashes(league);
			this.stashes = toStateful(flatten(tabs));
		},

		async fetchStash(stashId: string, league: League = ACTIVE_LEAGUE) {
			return stash(stashId, league);
		},

		deleteTabs() {
			this.stashes = [];
		},

		unselectAllTabs() {
			for (const stash of this.stashes) {
				stash.selected = false;
			}
		},
	},
});
