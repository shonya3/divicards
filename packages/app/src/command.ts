import { invoke } from '@tauri-apps/api';
import {
	DivinationCardsSample,
	League,
	DiscordIdentity,
	GoogleIdentity,
	CardNameAmount,
	TradeLeague,
	Result,
} from '@divicards/shared/types';
import { StashesResponseData, StashResponseData } from '@divicards/shared/poe.types';

export interface Commands {
	sample: (args: { csv: string; league: TradeLeague }) => Result<DivinationCardsSample>;
	sample_cards: (args: { cards: CardNameAmount[]; league: TradeLeague }) => Result<DivinationCardsSample>;
	merge: (args: { samples: DivinationCardsSample[] }) => DivinationCardsSample;
	league: (args: { sample: DivinationCardsSample; league: League }) => Result<DivinationCardsSample>;
	discord_auth: () => string;
	discord_identity: () => DiscordIdentity;
	discord_authenticated: () => boolean;
	discord_logout: () => void;
	google_auth: () => string;
	google_identity: () => GoogleIdentity;
	poe_auth: () => string;
	poe_logout: () => void;
	stashes: (args: { league: League }) => StashesResponseData;
	stash: (args: { league: League; stashId: string; subStashId?: string }) => StashResponseData;
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
