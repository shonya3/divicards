import { ref, computed } from 'vue';
import { command } from '../command';

export function useAppVersion() {
	const version = ref('');
	const tag = computed(() => `v${version.value}`);
	const releaseUrl = computed(() => `https://github.com/shonya3/divicards/releases/tag/${tag.value}`);

	async function checkVersion() {
		version.value = await command('version');
	}

	checkVersion();

	return { version, checkVersion, tag, releaseUrl };
}
