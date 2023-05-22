<script setup lang="ts">
import FileCard from './components/FileCard/FileCard.vue';
import { useFileCardsStore } from './stores/fileCards';
import { storeToRefs } from 'pinia';
import { initCustomFormatter, ref } from 'vue';
import { useAutoAnimate } from './composables/useAutoAnimate';
import { useDiscordOAuthStore } from './stores/discordOAuth';
import { command } from './command';

const filesStore = useFileCardsStore();
const { fileCards: files, selectedFiles, mergedFile } = storeToRefs(filesStore);
const { deleteFile, addCards, deleteAllFiles, merge, deleteMergedFile, downloadAll } = filesStore;

const filesTemplateRef = ref<HTMLElement | null>(null);
useAutoAnimate(filesTemplateRef);

const onDrop = (e: DragEvent) => {
	const dropFiles = e.dataTransfer?.files;
	if (dropFiles) addCards(Array.from(dropFiles));
};

// const { loggedIn, identity, name } = storeToRefs(useDiscordOAuthStore());
// const { login, logout, checkLoggedIn, init } = useDiscordOAuthStore();

// init();
</script>

<template>
	<!-- <div v-if="loggedIn">
		<p>{{ name }}</p>
		<button @click="logout">Logout</button>
	</div>
	<div v-else>
		<button @click="login">Login</button>
	</div> -->

	<div
		@drop.prevent="onDrop"
		@dragenter="(e: DragEvent) => e.preventDefault()"
		@dragover="(e: DragEvent) => e.preventDefault()"
		class="drag"
	>
		<div class="drop">Drop files <span>Here!</span></div>

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
