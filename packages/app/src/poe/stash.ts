import stashesDummy from './stashes.json' assert { type: 'json' };
import { StashTab, StatefulStashTab } from '@divicards/shared/poe.types';
const stashes = stashesDummy satisfies StashTab[];
const getStashes = (): Promise<StashTab[]> => new Promise(resolve => resolve(stashes));

export const flatten = (stashes: StashTab[]): StashTab[] => {
	let s: StashTab[] = [];

	for (const tab of stashes) {
		if (tab.type !== 'Folder') s.push(tab);
		if (tab.children) {
			for (const childTab of tab.children) {
				s.push(childTab);
			}
		}
	}
	return s;
};

export const toStateful = (stashes: StashTab[]): StatefulStashTab[] => {
	for (const tab of stashes) {
		(tab as StatefulStashTab).selected = false;
	}

	return stashes as StatefulStashTab[];
};
