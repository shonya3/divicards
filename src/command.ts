import { invoke } from '@tauri-apps/api';
import { DivinationCardsSample } from './types';

export type CommandOptions = {
	log: boolean;
};

export const command = async <Cmd extends keyof Commands>(
	cmd: Cmd,
	args: Commands[Cmd]['args'],
	options?: CommandOptions
): Promise<Commands[Cmd]['returns']> => {
	if (import.meta.env.DEV && !options) {
		options = { log: true };
	}

	if (options?.log) {
		const t0 = performance.now();
		const res = (await invoke(cmd, args)) as Commands[Cmd]['returns'];
		console.log(`${cmd}: ${new Intl.NumberFormat().format(performance.now() - t0)}ms`);
		return res;
	} else return invoke(cmd, args) as Promise<Commands[Cmd]['returns']>;
};

export interface Commands {
	sample: {
		args: { csv: string };
		returns: DivinationCardsSample;
	};

	chaos: {
		args: { sample: DivinationCardsSample; min: number };
		returns: number;
	};

	merge: {
		args: { samples: DivinationCardsSample[] };
		returns: DivinationCardsSample;
	};
}
