import { invoke } from '@tauri-apps/api';
import { DivinationCardsSample, League, CardNameAmount, TradeLeague, Result } from '@divicards/shared/types';
import { StashesResponseData, StashResponseData } from '@divicards/shared/poe.types';

type SampleData = string | CardNameAmount[];

export interface Commands {
	sample: (args: { data: SampleData; league: TradeLeague | null }) => Result<DivinationCardsSample>;
	merge: (args: { samples: DivinationCardsSample[] }) => DivinationCardsSample;
	open_url: (args: { url: string }) => void;
	poe_auth: () => string;
	poe_logout: () => void;
	stash: (args: { league: League; stashId: string; subStashId?: string }) => StashResponseData;
	stashes: (args: { league: League }) => StashesResponseData;
}

const { format } = new Intl.NumberFormat();
export const command = async <CommandName extends keyof Commands, Fn extends Commands[CommandName]>(
	name: CommandName,
	...args: Parameters<Fn>
): Promise<ReturnType<Fn>> => {
	if (import.meta.env.DEV) {
		const t0 = performance.now();
		const res = (await invoke(name, ...args)) as ReturnType<Fn>;
		// console.log(`${name}: ${format(performance.now() - t0)}ms`);
		return res;
	} else return invoke(name, ...args) as Promise<ReturnType<Fn>>;
};
