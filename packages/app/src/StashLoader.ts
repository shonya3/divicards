import { IStashLoader } from '@divicards/shared/IStashLoader.js';
import { DivinationCardsSample, League } from '@divicards/shared/types.js';
import { command } from './command';
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';

export class StashLoader implements IStashLoader {
	tab(tabId: string, league: string): Promise<TabWithItems> {
		return command('tab_with_items', { league, stashId: tabId });
	}

	sampleFromTab(tabId: string, league: League): Promise<DivinationCardsSample> {
		return command('sample_from_tab', { league, stashId: tabId });
	}

	/** Load items of a badge, supporting substash for Map/Gem/etc. */
	tabFromBadge(tab: NoItemsTab, league: League): Promise<TabWithItems> {
		if (tab.parent) {
			return command('tab_with_items', { league, stashId: tab.parent, subStashId: tab.id });
		}
		return command('tab_with_items', { league, stashId: tab.id });
	}

	/** Sample from badge (divination only) supporting substash. */
	sampleFromBadge(tab: NoItemsTab, league: League): Promise<DivinationCardsSample> {
		if (tab.parent) {
			return command('sample_from_tab', { league, stashId: tab.parent, subStashId: tab.id });
		}
		return command('sample_from_tab', { league, stashId: tab.id });
	}

	mapPrices(league: League): Promise<Array<{ name: string; tier: number; chaos_value: number | null }>> {
		return command('map_prices', { league });
	}

	currencyPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
		return command('currency_prices', { league });
	}

	fragmentPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
		return command('fragment_prices', { league });
	}

	essencePrices(league: League): Promise<Array<{ name: string; variant: string | null; chaos_value: number | null }>> {
		return command('essence_prices', { league });
	}

	gemPrices(league: League): Promise<Array<{ name: string; level: number; quality: number; chaos_value: number | null }>> {
		return command('gem_prices', { league });
	}

	oilPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
		return command('oil_prices', { league });
	}

    incubatorPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return command('incubator_prices', { league });
    }

	fossilPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
		return command('fossil_prices', { league });
	}

    resonatorPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return command('resonator_prices', { league });
    }

    divinationCardPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
        return command('divination_card_prices', { league });
    }

	deliriumOrbPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
		return command('delirium_orb_prices', { league });
	}

	vialPrices(league: League): Promise<Array<{ name: string; chaos_value: number | null }>> {
		return command('vial_prices', { league });
	}

	ninjaDenseOverviewsRaw(league: League): Promise<Record<string, unknown>> {
		return command('ninja_dense_overviews_raw', { league });
	}

	async tabs(league: League | string): Promise<NoItemsTab[]> {
		const { stashes = [] } = await command('stashes', { league });
		return this.#flattenStashes(stashes);
	}

	#flattenStashes(tabs: NoItemsTab[]): NoItemsTab[] {
		const flat: NoItemsTab[] = [];

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
