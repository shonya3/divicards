import { CommandList, command } from '../command';
import { computed, ref } from 'vue';

export const useCommand = <
	Cmd extends keyof CommandList,
	Args extends CommandList[Cmd]['args'],
	Return extends CommandList[Cmd]['returnType']
>(
	cmd: Cmd,
	args: Args,
	defaultValue: Awaited<Return>
) => {
	const data = ref(defaultValue);
	const error = ref('');
	const isError = computed(() => Boolean(error.value));
	const isReady = ref(false);

	const runCommand = async () => {
		isReady.value = false;
		error.value = '';
		try {
			data.value = await command(cmd, args);
			isReady.value = true;
		} catch (err) {
			error.value = err as string;
		}
	};

	runCommand();

	return {
		data,
		error,
		isError,
		isReady,
		runCommand,
	};
};
