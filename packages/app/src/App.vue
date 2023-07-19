<script setup lang="ts">
import FileCard from './components/FileCard/FileCard.vue';
import { useFileCardsStore } from './stores/fileCards';
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { useAutoAnimate } from './composables/useAutoAnimate';
import { usePoeOAuth2Store } from './stores/poeOAuth2Store';
import StashesMainComponent from './components/stashes/StashesMainComponent.vue';
import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { LeagueSelectElement } from '@divicards/wc/src/wc/league-select';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';
import { TabBadgeElement } from '@divicards/wc/src/wc/stashes/tab-badge';
DropFilesMessageElement.define();
LeagueSelectElement.define();
PoeAuthElement.define();

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
			<StashesMainComponent @close="stashVisible = false" />
		</div>

		<Transition>
			<div ref="filesTemplateRef" class="files" v-show="files.length">
				<FileCard
					v-for="file in files"
					v-bind="file"
					@delete="deleteFile(file.id)"
					v-model:selected="file.selected"
					v-model:minimumCardPrice.number="file.minimumCardPrice"
					v-model:league="file.league"
				/>
			</div>
		</Transition>

		<div v-if="files.length > 0">
			<h2>Select files you want to merge</h2>
			<button class="btn" @click="downloadAll">Download All</button>
			<button :disabled="selectedFiles.length < 2" class="btn" @click="merge">Merge samples</button>
			<button class="btn" @click="deleteAllFiles">Clear all</button>
		</div>
		<Transition>
			<FileCard
				v-if="mergedFile"
				v-bind="mergedFile"
				@delete="deleteMergedFile"
				v-model:minimumCardPrice.number="mergedFile.minimumCardPrice"
				v-model:league="mergedFile.league"
			/>
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