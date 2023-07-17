import { command } from '../command';
import { League } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';

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
	return stashes;
};
