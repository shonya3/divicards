<script setup lang="ts">
import { ref } from 'vue';

import { useFileCardsStore } from './stores/fileCards';
import { usePoeOAuth2Store } from './stores/poeOAuth2Store';
import { useAutoAnimate } from './composables/useAutoAnimate';

import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';
import { StashesViewElement } from '@divicards/wc/src/wc/stashes/stashes-view';
import { FileCardElement } from '@divicards/wc/src/wc/file-card/file-card';

import type { League } from '@divicards/shared/types';
import type { StashTab } from '@divicards/shared/poe.types';

StashesViewElement.define();
DropFilesMessageElement.define();
PoeAuthElement.define();
FileCardElement.define();

const fileCardsStore = useFileCardsStore();
const authStore = usePoeOAuth2Store();

const stashVisible = ref(false);
const filesTemplateRef = ref<HTMLElement | null>(null);
useAutoAnimate(filesTemplateRef);

const onDrop = async (e: DragEvent) => {
	for (const file of Array.from(e.dataTransfer?.files ?? [])) {
		fileCardsStore.addFromFile(file);
	}
};

const openStashWindow = async () => {
	if (authStore.loggedIn) {
		stashVisible.value = true;
	} else {
		try {
			await authStore.login();
			if (authStore.loggedIn) {
				stashVisible.value = true;
			}
		} catch (err) {
			console.log(err);
		}
	}
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
				@login="authStore.login"
				@logout="authStore.logout"
				:name="authStore.name"
				:loggedIn="authStore.loggedIn"
			></wc-poe-auth>
		</header>

		<div v-show="authStore.loggedIn && stashVisible">
			<wc-stashes-view
				@tab-data="
					async (e: CustomEvent<{ league: League; tab: StashTab }>) =>
						fileCardsStore.addFromTab(e.detail.league, e.detail.tab)
				"
				@close="stashVisible = false"
			></wc-stashes-view>
		</div>

		<Transition>
			<div ref="filesTemplateRef" class="files" v-show="fileCardsStore.fileCards.length">
				<wc-file-card
					v-for="fileCard in fileCardsStore.fileCards"
					v-bind="fileCard"
					@delete="(e: CustomEvent<string>) => fileCardsStore.deleteFile(e.detail)"
					@upd:league="(e: CustomEvent<League>) => fileCardsStore.replaceFileCard(e.detail, fileCard)"
					@upd:selected="(e: CustomEvent<boolean>) => fileCard.selected = e.detail"
					@upd:minimumCardPrice="(e: CustomEvent<number>) => fileCard.minimumCardPrice = e.detail"
				></wc-file-card>
			</div>
		</Transition>

		<div v-if="fileCardsStore.fileCards.length > 0">
			<h2>Select files you want to merge</h2>
			<button class="btn" @click="fileCardsStore.downloadAll">Download All</button>
			<button :disabled="fileCardsStore.selectedFiles.length < 2" class="btn" @click="fileCardsStore.merge">
				Merge samples
			</button>
			<button class="btn" @click="fileCardsStore.deleteAllFiles">Clear all</button>
		</div>
		<Transition>
			<wc-file-card
				v-if="fileCardsStore.mergedFile"
				v-bind="fileCardsStore.mergedFile"
				@delete="fileCardsStore.deleteMergedFile"
				@upd:minimumCardPrice="(e: CustomEvent<number>) => fileCardsStore.mergedFile && (fileCardsStore.mergedFile.minimumCardPrice = e.detail)"
				@upd:league="(e: CustomEvent<League>) => fileCardsStore.replaceMerged(e.detail)"
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
