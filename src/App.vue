<script setup lang="ts">
import FileCard from './components/FileCard/FileCard.vue';
import { useFileCardsStore } from './stores/fileCards';
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { useAutoAnimate } from './composables/useAutoAnimate';
import { usePoeOAuth2Store } from './stores/poeOAuth2Store';
import PoeAuth from './components/PoeAuth.vue';
import StashesMainComponent from './components/stashes/StashesMainComponent.vue';
import DropFilesMessage from './components/DropFilesMessage.vue';

const filesStore = useFileCardsStore();
const { fileCards: files, selectedFiles, mergedFile } = storeToRefs(filesStore);
const { deleteFile, addCards, deleteAllFiles, merge, deleteMergedFile, downloadAll } = filesStore;

const poeOAuthStore = usePoeOAuth2Store();
const { loggedIn } = storeToRefs(poeOAuthStore);

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
			<DropFilesMessage />
			<button @click="openStashWindow()">Load from stash</button>
			<PoeAuth />
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
				v-model:selected="mergedFile.selected"
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
