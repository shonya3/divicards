import { invoke } from '@tauri-apps/api';
import { DivinationCardsSample } from './types';

export const command = async <T extends keyof CommandList>(
	cmd: T,
	args: CommandList[T]['args']
): Promise<CommandList[T]['returnType']> => {
	return invoke(cmd, args) as Promise<CommandList[T]['returnType']>;
};

export interface CommandList {
	update_prices: {
		args: {};
		returnType: void;
	};

	sample: {
		args: { csv: string };
		returnType: DivinationCardsSample;
	};

	chaos: {
		args: { csv: string; min: number };
		returnType: number;
	};

	merge: {
		args: { samples: DivinationCardsSample[] };
		returnType: DivinationCardsSample;
	};
}
