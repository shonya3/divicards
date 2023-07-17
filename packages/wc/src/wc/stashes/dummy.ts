import { League } from '@divicards/shared/types';
import { StatefulStashTab } from '@divicards/shared/poe.types';
import json from './tabBadgeGroupProps.json' assert { type: 'json' };
export const tabBadgeProps = json as { league: League; stashes: StatefulStashTab[] };
