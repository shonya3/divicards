import { DivinationCardsSample, League } from '@divicards/shared/types.js';
import { IStashLoader } from '@divicards/shared/IStashLoader.js';

import stashesData from './json/stashes.json' with { type: 'json' };
import sampleData from './json/sample.json' with { type: 'json' };
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';

export const stashes = stashesData as NoItemsTab[];
export const league: League = 'Standard';
export const sample: DivinationCardsSample = sampleData;
import quadStash from './json/QuadStashStd.json'  with { type: 'json' };
import fragmentsStash from './json/fragmentsTab.json'  with { type: 'json' };
const quad = quadStash as TabWithItems;
const fragments = fragmentsStash as TabWithItems;

const sleepSecs = (secs: number): Promise<void> => new Promise(resolve => setTimeout(resolve, secs * 1000));

let stash: 'quad' | 'fragments' = 'quad';
export class MockStashLoader implements IStashLoader {
    async tab(_tabId: string, _league: string): Promise<TabWithItems> {
        const nextStash = stash === 'quad' ? fragments : quad;
        stash = stash === 'quad' ? 'fragments' : 'quad';
        await sleepSecs(0.2);
        return nextStash;
    }
    async tabFromBadge(_tab: NoItemsTab, _league: League): Promise<TabWithItems> {
        return this.tab('', _league);
    }
    sampleFromTab(_tabId: string, _league: League): Promise<DivinationCardsSample> {
        return new Promise(r =>
            setTimeout(() => {
                r(sample);
            }, 50)
        );
    }
    sampleFromBadge(_tab: NoItemsTab, _league: League): Promise<DivinationCardsSample> {
        return this.sampleFromTab('', _league);
    }
    async mapPrices(_league: League): Promise<Array<{ name: string; tier: number; chaos_value: number | null }>> {
        return [];
    }
    async currencyPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async fragmentPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async essencePrices(_league: League): Promise<Array<{ name: string; variant: string | null; chaos_value: number | null }>> {
        return [];
    }
    async gemPrices(_league: League): Promise<Array<{ name: string; level: number; quality: number; chaos_value: number | null }>> {
        return [];
    }
    async oilPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async incubatorPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async fossilPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async resonatorPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async deliriumOrbPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async vialPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async divinationCardPrices(_league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return [];
    }
    async ninjaDenseOverviewsRaw(_league: League): Promise<Record<string, unknown>> {
        return {};
    }
    tabs(_league: League): Promise<NoItemsTab[]> {
        return new Promise(r => r(stashes));
    }
}
