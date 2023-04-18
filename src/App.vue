<script setup lang="ts">
import FileCard from './components/FileCard/FileCard.vue';
import { useFileCardsStore } from './stores/fileCards';
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { useAutoAnimate } from './composables/useAutoAnimate';

const filesStore = useFileCardsStore();
const { fileCards: files, selectedFiles, mergedFile } = storeToRefs(filesStore);
const { deleteFile, addCards, deleteAllFiles, merge, deleteMergedFile, downloadAll, updateAllCardsPrice } = filesStore;

const filesTemplateRef = ref<HTMLElement | null>(null);
useAutoAnimate(filesTemplateRef);

const onDrop = (e: DragEvent) => {
	const dropFiles = e.dataTransfer?.files;
	if (dropFiles) addCards(Array.from(dropFiles));
};
</script>

<template>
	<div @drop.prevent="onDrop" @dragenter="e => e.preventDefault()" @dragover="e => e.preventDefault()" class="drag">
		<div class="drop">Drop files <span>Here!</span></div>

		<Transition>
			<div ref="filesTemplateRef" class="files" v-show="files.length">
				<FileCard
					v-for="file in files"
					v-bind="file"
					@delete-me="deleteFile(file.id)"
					v-model:selected="file.selected"
					v-model:minimumCardPrice.number="file.minimumCardPrice"
					@update:minimum-card-price="newMinPrice => updateAllCardsPrice(file.id, newMinPrice)"
				/>
			</div>
		</Transition>

		<div v-if="files.length > 0">
			<h2>Select files you want to merge</h2>
			<button class="btn" @click="downloadAll">Download All</button>
			<button :disabled="!selectedFiles.length" class="btn" @click="merge">Merge CSV</button>
			<button class="btn" @click="deleteAllFiles">Clear all</button>
		</div>
		<Transition>
			<FileCard
				v-if="mergedFile"
				v-bind="mergedFile"
				@delete-me="deleteMergedFile"
				v-model:selected="mergedFile.selected"
				v-model:minimumCardPrice.number="mergedFile.minimumCardPrice"
				@update:minimum-card-price="newMinPrice => updateAllCardsPrice(mergedFile!.id, newMinPrice)"
			/>
		</Transition>
	</div>
</template>

<style scoped>
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
.drop {
	font-size: 3rem;
	margin-bottom: 1rem;
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

.drop > span {
	color: deeppink;
	font-weight: 700;
}
</style>
