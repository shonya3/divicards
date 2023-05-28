import { StatefulStashTab } from '../../stores/stash';
import { League, permanentLeagues } from '../../types';
export const REMOVE_ONLY = '(Remove-only)';

export const paginate = (stashes: StatefulStashTab[], page: number, perPage: number) => {
	const start = (page - 1) * perPage;
	const end = start + perPage;
	return stashes.slice(start, end);
};

export const filter = (
	stashes: StatefulStashTab[],
	nameQuery: string,
	shouldFilter: boolean,
	hideRemoveOnly: boolean
): StatefulStashTab[] => {
	if (!shouldFilter) return stashes;

	return stashes.filter(({ name }) => {
		if (hideRemoveOnly) {
			if (name.includes(REMOVE_ONLY)) return false;
		}
		return name.toLowerCase().includes(nameQuery.toLowerCase());
	});
};

export const shouldUnlockHideRemoveOnly = (league: League, filteredStashes: StatefulStashTab[]) => {
	return permanentLeagues.includes(league) && filteredStashes.some(({ name }) => name.includes(REMOVE_ONLY));
};
