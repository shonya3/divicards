<script setup lang="ts">
import { ref } from 'vue';

import { useFileCardsStore } from './stores/fileCards';
import { useAuthStore } from './stores/auth';
import { useAutoAnimate } from './composables/useAutoAnimate';

import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';

import FileCard from './components/FileCard.vue';
import StashesView from './components/StashesView.vue';

DropFilesMessageElement.define();
PoeAuthElement.define();

const fileCardsStore = useFileCardsStore();
const authStore = useAuthStore();

const stashVisible = ref(false);
const filesTemplateRef = ref<HTMLElement | null>(null);
useAutoAnimate(filesTemplateRef);

const onDrop = async (e: DragEvent) => {
	for (const file of Array.from(e.dataTransfer?.files ?? [])) {
		fileCardsStore.addFromFile(file);
	}
};

const openStashWindow = async () => {
	if (!authStore.loggedIn) {
		await authStore.login();
	}

	stashVisible.value = true;
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
			<StashesView @tab-tada="fileCardsStore.addFromTab" @close="stashVisible = false" />
		</div>

		<Transition>
			<div ref="filesTemplateRef" class="files" v-show="fileCardsStore.fileCards.length">
				<FileCard
					v-for="fileCard in fileCardsStore.fileCards"
					v-bind="fileCard"
					@delete="fileCardsStore.deleteFile"
					v-model:selected="fileCard.selected"
					v-model:minimum-card-price="fileCard.minimumCardPrice"
					@update:league="league => fileCardsStore.replaceFileCard(league, fileCard)"
				/>
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
			<FileCard
				v-if="fileCardsStore.mergedFile"
				v-bind="fileCardsStore.mergedFile"
				@delete="fileCardsStore.deleteMergedFile"
				@update:minimum-card-price="
					price => fileCardsStore.mergedFile && (fileCardsStore.mergedFile.minimumCardPrice = price)
				"
				@update:league="fileCardsStore.replaceMerged"
			/>
		</Transition>
	</div>
</template>

<style scoped>
.header {
	display: flex;
	justify-content: space-between;
	align-items: center;
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
./stores/poeAuth ./stores/auth
