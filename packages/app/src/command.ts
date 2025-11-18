import { invoke } from '@tauri-apps/api/core';
import {
	DivinationCardsSample,
	League,
	NameAmount,
	TradeLeague,
	GoogleIdentity,
	TablePreferences,
	Column,
} from '@divicards/shared/types.js';
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';

export type SampleData = string | NameAmount[] | DivinationCardsSample;
export type ValueRange = {
	majorDimension: 'ROWS' | 'COLUMNS';
	range: string;
	values: Array<Array<string | number | null | undefined>>;
};
type Preferences = Omit<TablePreferences, 'columns'> & { columns: Column[] };

export interface Commands {
	version: () => string;
	read_batch(args: { spreadsheetId: string; ranges: string[] }): unknown;
	read_sheet(args: { spreadsheetId: string; range: string }): ValueRange;
	new_sheet_with_sample: (args: {
		spreadsheetId: string;
		title: string;
		sample: DivinationCardsSample;
		league: League;
		preferences: Preferences;
	}) => string;
	google_logout: () => void;
	google_identity: () => GoogleIdentity;
	google_auth: () => void;
	old_google_auth: () => void;
	sample: (args: { data: SampleData; league: TradeLeague | null }) => DivinationCardsSample;
	merge: (args: { samples: DivinationCardsSample[] }) => DivinationCardsSample;
	open_url: (args: { url: string }) => void;
	poe_auth: () => string;
	poe_logout: () => void;
	stashes: (args: { league: League }) => { stashes: NoItemsTab[] };
	sample_into_csv: (args: { sample: DivinationCardsSample; preferences: Preferences }) => string;
	sample_from_tab: (args: { league: League; stashId: string; subStashId?: string }) => DivinationCardsSample;
	tab_with_items: (args: { league: League; stashId: string; subStashId?: string }) => TabWithItems;
	extract_cards: (args: { tab: TabWithItems; league: League }) => DivinationCardsSample;
	map_prices: (args: { league: League }) => Array<{ name: string; tier: number; chaos_value: number | null }>;
	currency_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
		fragment_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
		essence_prices: (args: { league: League }) => Array<{ name: string; variant: string | null; chaos_value: number | null }>;
		gem_prices: (args: { league: League }) => Array<{ name: string; level: number; quality: number; chaos_value: number | null }>;
		oil_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
		incubator_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
		fossil_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
		resonator_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
		delirium_orb_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
        vial_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
        divination_card_prices: (args: { league: League }) => Array<{ name: string; chaos_value: number | null }>;
        ninja_dense_overviews_raw: (args: { league: League }) => Record<string, unknown>;
		set_gem_prices_cache_ttl_minutes: (args: { minutes: number }) => void;
}

const { format } = new Intl.NumberFormat();

const debug = false;
export const command = async <CommandName extends keyof Commands, Fn extends Commands[CommandName]>(
    name: CommandName,
    ...args: Parameters<Fn>
): Promise<ReturnType<Fn>> => {
    const isTauri =
        (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ != null) ||
        (typeof navigator !== 'undefined' && navigator.userAgent.includes('Tauri')) ||
        (typeof import.meta !== 'undefined' && (import.meta as any).env && ((import.meta as any).env.TAURI_PLATFORM ?? (import.meta as any).env.TAURI));
    if (isTauri) {
        if (debug) {
            const t0 = performance.now();
            const res = (await invoke(name, ...args)) as ReturnType<Fn>;
            console.log(`${name}: ${format(performance.now() - t0)}ms`);
            return res;
        } else return invoke(name, ...args) as Promise<ReturnType<Fn>>;
    }
    const r = await mockInvoke(name as string, args[0] as Record<string, unknown>);
    return r as ReturnType<Fn>;
};

async function mockInvoke(name: string, arg: Record<string, unknown>): Promise<unknown> {
    switch (name) {
        case 'version':
            return 'dev-web';
        case 'stashes':
            return {
                stashes: [
                    { id: 'tabA', index: 0, name: 'Tab A', type: 'NormalStash', selected: false },
                    { id: 'tabB', index: 1, name: 'Tab B', type: 'NormalStash', selected: false },
                ],
            };
        case 'tab_with_items': {
            const id = (arg?.stashId as string) ?? 'tabA';
            const idx = id === 'tabA' ? 0 : 1;
            const league = (arg?.league as string) ?? 'Standard';
            const items = [
                {
                    typeLine: 'Chaos Orb',
                    baseType: 'Chaos Orb',
                    stackSize: 12,
                    w: 1,
                    h: 1,
                    x: 0,
                    y: 0,
                    identified: true,
                    league,
                },
                {
                    typeLine: 'Vaal Orb',
                    baseType: 'Vaal Orb',
                    stackSize: 5,
                    w: 1,
                    h: 1,
                    x: 1,
                    y: 0,
                    identified: true,
                    league,
                },
                {
                    typeLine: 'Ancient Orb',
                    baseType: 'Ancient Orb',
                    stackSize: 2,
                    w: 1,
                    h: 1,
                    x: 2,
                    y: 0,
                    identified: true,
                    league,
                },
            ];
            return { id, index: idx, name: id, type: 'NormalStash', items };
        }
        case 'sample_from_tab': {
            const sample = {
                cards: [],
                notCards: [],
                fixedNames: [],
            } as const;
            return sample;
        }
        case 'sample_into_csv': {
            const prefs = (arg?.preferences ?? {}) as { columns?: string[] };
            const cols = (prefs.columns ?? ['name', 'amount']) as string[];
            const sample = (arg?.sample ?? {}) as { cards?: Array<Record<string, any>> };
            const cards = Array.isArray(sample.cards) ? sample.cards : [];
            const header = cols.join(',');
            const lines = cards.map(c =>
                cols
                    .map(k => {
                        const v = c[k];
                        if (v === null || v === undefined) return '';
                        return typeof v === 'string' ? v.replaceAll(',', ';') : String(v);
                    })
                    .join(',')
            );
            return [header, ...lines].join('\n');
        }
        case 'map_prices':
            return [];
        case 'currency_prices':
            return [
                { name: 'Chaos Orb', chaos_value: 60 },
                { name: 'Vaal Orb', chaos_value: 5 },
                { name: 'Ancient Orb', chaos_value: 15 },
            ];
        case 'fragment_prices':
            return [];
        case 'essence_prices':
            return [];
        case 'gem_prices':
            return [];
        case 'oil_prices':
            return [];
        case 'incubator_prices':
            return [];
        case 'fossil_prices':
            return [];
        case 'resonator_prices':
            return [];
        case 'delirium_orb_prices':
            return [];
        case 'vial_prices':
            return [];
        case 'divination_card_prices':
            return [];
        case 'ninja_dense_overviews_raw':
            return {};
        case 'set_gem_prices_cache_ttl_minutes':
            return;
        case 'google_logout':
            return;
        case 'google_identity':
            return { name: '', email: '', picture: '' };
        case 'google_auth':
            return;
        case 'old_google_auth':
            return;
        case 'open_url':
            return;
        case 'poe_auth':
            return '';
        case 'poe_logout':
            return;
        case 'sample': {
            const sample = {
                cards: [],
                notCards: [],
                fixedNames: [],
            } as const;
            return sample;
        }
        case 'merge': {
            const sample = {
                cards: [],
                notCards: [],
                fixedNames: [],
            } as const;
            return sample;
        }
        default:
            throw new Error(`Unsupported command: ${name}`);
    }
}
