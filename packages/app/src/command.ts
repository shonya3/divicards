import { invoke } from '@tauri-apps/api/core';
import {
	DivinationCardsSample,
	League,
	NameAmount,
	TradeLeague,
	GoogleIdentity,
	TablePreferences,
	Column,
} from '@divicards/shared/types';
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
	tab_with_items: (args: { league: League; stashId: string }) => TabWithItems;
	extract_cards: (args: { tab: TabWithItems; league: League }) => DivinationCardsSample;
}

const { format } = new Intl.NumberFormat();

const debug = false;
export const command = async <CommandName extends keyof Commands, Fn extends Commands[CommandName]>(
	name: CommandName,
	...args: Parameters<Fn>
): Promise<ReturnType<Fn>> => {
	if (debug) {
		const t0 = performance.now();
		const res = (await invoke(name, ...args)) as ReturnType<Fn>;
		console.log(`${name}: ${format(performance.now() - t0)}ms`);
		return res;
	} else return invoke(name, ...args) as Promise<ReturnType<Fn>>;
};
