import { invoke } from '@tauri-apps/api';
import { DivinationCardsSample, League, DiscordIdentity, GoogleIdentity, leagues, CardNameAmount } from './types';
import { StashesResponseData, StashResponseData } from './poe/types';

export interface Commands {
	sample: (args: { csv: string; league: League }) => DivinationCardsSample;
	sample_cards: (args: { cards: CardNameAmount[]; league: League }) => DivinationCardsSample;
	chaos: (args: { sample: DivinationCardsSample; min: number }) => number;
	merge: (args: { samples: DivinationCardsSample[] }) => DivinationCardsSample;
	league: (args: { sample: DivinationCardsSample; league: League }) => DivinationCardsSample;
	discord_auth: () => string;
	discord_identity: () => DiscordIdentity;
	discord_authenticated: () => boolean;
	discord_logout: () => void;
	google_auth: () => string;
	google_identity: () => GoogleIdentity;
	poe_auth: () => string;
	poe_authenticated: () => boolean;
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
		console.log(`${name}: ${format(performance.now() - t0)}ms`);
		return res;
	} else return invoke(name, ...args) as Promise<ReturnType<Fn>>;
};
