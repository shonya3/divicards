import { invoke } from '@tauri-apps/api';
import { DivinationCardsSample, League } from './types';

export type CommandOptions = {
	log: boolean;
};

export interface Commands {
	sample: (args: { csv: string; league: League }) => DivinationCardsSample;
	chaos: (args: { sample: DivinationCardsSample; min: number }) => number;
	merge: (args: { samples: DivinationCardsSample[] }) => DivinationCardsSample;
	league: (args: { sample: DivinationCardsSample; league: League }) => DivinationCardsSample;
}

export const command = async <Cmd extends keyof Commands>(
	cmd: Cmd,
	args: Parameters<Commands[Cmd]>[0],
	options?: CommandOptions
): Promise<ReturnType<Commands[Cmd]>> => {
	if (import.meta.env.DEV && !options) {
		options = { log: true };
	}

	if (options?.log) {
		const t0 = performance.now();
		const res = (await invoke(cmd, args)) as ReturnType<Commands[Cmd]>;
		console.log(`${cmd}: ${new Intl.NumberFormat().format(performance.now() - t0)}ms`);
		return res;
	} else return invoke(cmd, args) as Promise<ReturnType<Commands[Cmd]>>;
};
