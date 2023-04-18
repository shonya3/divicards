<script setup lang="ts">
import FileCard from './components/FileCard/FileCard.vue';
import { useFileCardsStore } from './stores/fileCards';
import { storeToRefs } from 'pinia';

const filesStore = useFileCardsStore();
const { fileCards: files, selectedFiles, mergedFile } = storeToRefs(filesStore);
const { deleteFile, addCards, deleteAllFiles, merge, deleteMergedFile, downloadAll } = filesStore;

const onDrop = (e: DragEvent) => {
	const dropFiles = e.dataTransfer?.files;
	if (dropFiles) addCards(Array.from(dropFiles));
};
</script>

<template>
	<div @drop.prevent="onDrop" @dragenter="e => e.preventDefault()" @dragover="e => e.preventDefault()" class="drag">
		<div class="drop">Drop files <span>Here!</span></div>

		<Transition>
			<div v-if="files.length" class="files" v-show="files.length">
				<FileCard
					v-for="file in files"
					v-bind="file"
					@update:selected="e => file.selected"
					@minimum-price-updated="p => (file.data.allCardsPrice = p)"
					@delete-me="deleteFile(file.id)"
					v-model:selected="file.selected"
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
				:minimum-card-price="mergedFile.minimumCardPrice"
				@update:selected="e => mergedFile!.selected"
				@minimum-price-updated="p => (mergedFile!.data.allCardsPrice = p)"
				@delete-me="deleteMergedFile"
				v-model:selected="mergedFile.selected"
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
