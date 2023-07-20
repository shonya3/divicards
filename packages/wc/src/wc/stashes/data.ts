import { League } from '@divicards/shared/types';
import json from './tabBadgeGroupProps.json' assert { type: 'json' };
import { StashTab } from '@divicards/shared/poe.types';
export const { league, stashes } = json as { league: League; stashes: StashTab[] };
