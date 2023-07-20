import { command } from '../command';
import { League } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { StashTab } from '@divicards/shared/poe.types';

export const auth = async () => {
	return command('poe_auth');
};

export const logout = async () => {
	return command('poe_logout');
};

export const stash = async (stashId: string, league: League = ACTIVE_LEAGUE) => {
	const responseData = await command('stash', {
		league,
		stashId,
	});
	const tab = responseData.stash ?? [];
	return tab;
};

export const stashes = async (league: League = ACTIVE_LEAGUE) => {
	const { stashes = [] } = await command('stashes', { league });
	return flattenStashes(stashes);
};

export const flattenStashes = (stashes: StashTab[]): StashTab[] => {
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

import stashesDummy from './stashes.json' assert { type: 'json' };
export const stashesData: StashTab[] = stashesDummy;
export const getStashes = (): Promise<StashTab[]> => new Promise(resolve => resolve(stashesData));
