<script setup lang="ts">
import { useFileCardsStore } from './stores/fileCards';
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useAutoAnimate } from './composables/useAutoAnimate';
import { usePoeOAuth2Store } from './stores/poeOAuth2Store';
import StashesMainComponent from './components/stashes/StashesMainComponent.vue';
import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { LeagueSelectElement } from '@divicards/wc/src/wc/league-select';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';
import { TabBadgeElement } from '@divicards/wc/src/wc/stashes/tab-badge';
import { FileCardElement, FileCardProps } from '@divicards/wc/src/wc/file-card/file-card';
import { League, isTradeLeague } from '@divicards/shared/types';
import { StashesViewElement } from '../../wc/src/wc/stashes/stashes-view';
import { StashTab } from '@divicards/shared/poe.types';
import { command } from './command';
import { cardsFromTab } from './poe/cards';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
StashesViewElement.define();
DropFilesMessageElement.define();
LeagueSelectElement.define();
PoeAuthElement.define();
FileCardElement.define();

TabBadgeElement.define();

const filesStore = useFileCardsStore();
const { fileCards: files, selectedFiles, mergedFile } = storeToRefs(filesStore);
const { deleteFile, addCards, deleteAllFiles, merge, deleteMergedFile, downloadAll } = filesStore;

const poeOAuthStore = usePoeOAuth2Store();
const { loggedIn, name } = storeToRefs(poeOAuthStore);

const filesTemplateRef = ref<HTMLElement | null>(null);
useAutoAnimate(filesTemplateRef);

const onDrop = (e: DragEvent) => {
	const dropFiles = e.dataTransfer?.files;
	if (dropFiles) addCards(Array.from(dropFiles));
};

const stashVisible = ref(false);
const openStashWindow = () => {
	if (loggedIn.value) {
		stashVisible.value = true;
	} else {
		poeOAuthStore.login().then(() => {
			if (loggedIn.value) {
				stashVisible.value = true;
			}
		});
	}
};

const onUpdateSelected = (e: CustomEvent<boolean>, fileCard: FileCardProps) => {
	fileCard.selected = e.detail;
};

const onUpdateLeague = (e: CustomEvent<League>, fileCard: FileCardProps) => {
	fileCard.league = e.detail;
};

const onUpdateMinimumPrice = (e: CustomEvent<number>, fileCard: FileCardProps) => {
	fileCard.minimumCardPrice = e.detail;
};

const onUpdateMergedMinimumPrice = (e: CustomEvent<number>) => {
	if (!filesStore.mergedFile) return;
	filesStore.mergedFile.minimumCardPrice = e.detail;
};

const onUpdateMergedLeague = (e: CustomEvent<League>) => {
	if (!filesStore.mergedFile) return;
	filesStore.mergedFile.league = e.detail;
};

const onTabData = async (e: CustomEvent<{ league: League; tab: StashTab }>) => {
	const { league, tab } = e.detail;
	const tradeLeague = isTradeLeague(league) ? league : ACTIVE_LEAGUE;
	console.log('tab-data from App.vue', tab);

	const sample = await command('sample_cards', {
		cards: cardsFromTab(tab),
		league: tradeLeague,
	});
	const file = new File([sample.polished], `${tab.name}.csv`);
	filesStore.addCards([file], tradeLeague);
};
</script>

<template>
	<div
		@drop.prevent="onDrop"
		@dragenter="(e: DragEvent) => e.preventDefault()"
		@dragover="(e: DragEvent) => e.preventDefault()"
		class="drag"
	>
		<header class="header">
			<wc-drop-files-message></wc-drop-files-message>
			<button @click="openStashWindow()">Load from stash</button>
			<wc-poe-auth
				@login="poeOAuthStore.login"
				@logout="poeOAuthStore.logout"
				:name="name"
				:loggedIn="loggedIn"
			></wc-poe-auth>
		</header>

		<div v-show="loggedIn && stashVisible">
			<wc-stashes-view @tab-data="onTabData" @close="stashVisible = false"></wc-stashes-view>
		</div>

		<Transition>
			<div ref="filesTemplateRef" class="files" v-show="files.length">
				<wc-file-card
					v-for="fileCardProps in files"
					v-bind="fileCardProps"
					@delete="(e: CustomEvent<string>) => deleteFile(e.detail)"
					@upd:league="(e: CustomEvent<League>) => onUpdateLeague(e, fileCardProps)"
					@upd:selected="(e: CustomEvent<boolean>) => onUpdateSelected(e, fileCardProps)"
					@upd:minimumCardPrice="(e: CustomEvent<number>) => onUpdateMinimumPrice(e, fileCardProps)"
				></wc-file-card>
			</div>
		</Transition>

		<div v-if="files.length > 0">
			<h2>Select files you want to merge</h2>
			<button class="btn" @click="downloadAll">Download All</button>
			<button :disabled="selectedFiles.length < 2" class="btn" @click="merge">Merge samples</button>
			<button class="btn" @click="deleteAllFiles">Clear all</button>
		</div>
		<Transition>
			<wc-file-card
				v-if="mergedFile"
				v-bind="mergedFile"
				@delete="deleteMergedFile"
				@upd:minimumCardPrice="onUpdateMergedMinimumPrice"
				@upd:league="onUpdateMergedLeague"
			></wc-file-card>
		</Transition>
	</div>
</template>

<style scoped>
.header {
	display: flex;
	justify-content: space-between;
	margin-bottom: 3rem;
}

.v-enter-active,
.v-leave-active {
	transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
	opacity: 0;
}

.drag {
	height: 100vh;
	position: relative;
	padding: 1rem;
	min-width: 800px;

	display: flex;
	flex-direction: column;
	gap: 2rem;
}
.drag--active {
	filter: hue-rotate(120deg);
}

.files {
	display: flex;
	flex-wrap: wrap;
	gap: 2rem;
}

.btn {
	margin-left: 2rem;
	padding: 0.4rem;
	font-size: 1.4rem;
}
</style>
