import { invoke } from '@tauri-apps/api';
import {
	DivinationCardsSample,
	League,
	CardNameAmount,
	TradeLeague,
	GoogleIdentity,
	TablePreferences,
	Column,
} from '@divicards/shared/types';
import { NoItemsTab } from '@divicards/shared/poe.types';

export type SampleData = string | CardNameAmount[];

export interface Commands {
	add_sheet_with_sample: (args: {
		spreadsheetId: string;
		title: string;
		sample: DivinationCardsSample;
		preferences: Omit<TablePreferences, 'columns'> & { columns: Column[] };
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
	sample_from_tab: (args: { league: League; stashId: string; subStashId?: string }) => DivinationCardsSample;
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
