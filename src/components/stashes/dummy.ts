import json from './tabBadgeGroupProps.json' assert { type: 'json' };
import { League } from '../../types';
import { StatefulStashTab } from '../../stores/stash';
export const tabBadgeProps = json as { league: League; stashes: StatefulStashTab[] };
