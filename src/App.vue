<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Contents } from './types';
import { downloadFiles, createCsvFile, createContents, command } from './lib';
import FileCard from './components/FileCard/FileCard.vue';
import autoAnimate from '@formkit/auto-animate';

const filesEl = ref<HTMLElement | null>(null);
const mainContents = ref<Contents[]>([]);
const validContents = computed(() => mainContents.value.filter(({ valid }) => valid));
const validSelectedContents = computed(() => mainContents.value.filter(({ valid, selected }) => valid && selected));
const selectedContents = computed(() => mainContents.value.filter(({ selected }) => selected));
const validSelectedStrings = computed(() => validSelectedContents.value.map(({ fileContent }) => fileContent.text));

const onDrop = (e: DragEvent) => {
	e.preventDefault();
	if (!e.dataTransfer) return;
	const { files } = e.dataTransfer;
	Array.from(files).forEach(async file => {
		try {
			const contents = await createContents(file);
			mainContents.value.push(contents);
		} catch (err) {
			console.log('Error creating contents: ', err);
		}
	});
};

const onDelete = (id: string) => {
	mainContents.value = mainContents.value.filter(contents => contents.id !== id);
};

const mergedContents = ref<Contents | null>(null);
const updateMergeFile = async () => {
	const mergedCsv = await command('merge_csv', { csvFileStrings: validSelectedStrings.value });
	const file = createCsvFile(mergedCsv, 'merged.csv');
	mergedContents.value = null;
	const contents = await createContents(file);
	contents.selected = null;
	mergedContents.value = contents;
};

onMounted(() => {
	if (filesEl.value instanceof HTMLElement) {
		autoAnimate(filesEl.value);
	}
});
</script>

<template>
	<div @drop="onDrop" @dragenter="e => e.preventDefault()" @dragover="e => e.preventDefault()" class="drag">
		<div class="drop">Drop files <span>Here!</span></div>
		<Transition>
			<div ref="filesEl" class="files" v-show="mainContents.length">
				<FileCard
					v-for="contents in mainContents"
					:key="contents.id"
					v-bind="contents"
					@update:selected="e => contents.selected"
					@minimum-price-updated="p => (contents.price = p)"
					@delete-me="onDelete"
					v-model:selected="contents.selected"
				/>
			</div>
		</Transition>

		<div v-if="mainContents.length > 0">
			<h2>Select files you want to merge</h2>
			<button class="btn" @click="downloadFiles(validContents.map(c => c.fileContent))">Download All</button>
			<button :disabled="!validSelectedContents.length" class="btn" @click="updateMergeFile">Merge CSV</button>
			<button class="btn" @click="mainContents = []">Clear all</button>
		</div>
		<Transition>
			<FileCard
				v-if="mergedContents"
				v-bind="mergedContents"
				v-model:selected="mergedContents.selected"
				@delete-me="mergedContents = null"
				@minimum-price-updated="
					p => {
						mergedContents && (mergedContents.price = p);
					}
				"
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
