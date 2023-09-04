<script setup lang="ts">
import { ref } from 'vue';

import { useSampleStore } from './stores/sample';
import { useAuthStore } from './stores/auth';
import { useAutoAnimate } from './composables/useAutoAnimate';

import SampleCard from './components/SampleCard.vue';
import StashesView from './components/StashesView.vue';
import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';
import { StashLoader } from './StashLoader';
DropFilesMessageElement.define();
PoeAuthElement.define();
const stashLoader = new StashLoader();

const sampleStore = useSampleStore();
const authStore = useAuthStore();

const stashVisible = ref(false);
const samplesContainerRef = ref<HTMLElement | null>(null);
useAutoAnimate(samplesContainerRef);

const openStashWindow = async () => {
	if (!authStore.loggedIn) {
		await authStore.login();
	}

	stashVisible.value = true;
};
</script>

<template>
	<div
		@drop.prevent="sampleStore.addFromDragAndDrop"
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
			<StashesView
				:stashLoader="stashLoader"
				@sample-from-tab="sampleStore.addSample"
				@close="stashVisible = false"
			/>
		</div>

		<Transition>
			<SampleCard
				v-if="sampleStore.merged"
				v-bind="sampleStore.merged"
				@delete="sampleStore.deleteMerged"
				@update:minimumCardPrice="price => sampleStore.merged && (sampleStore.merged.minimumCardPrice = price)"
				@update:league="sampleStore.replaceMerged"
			/>
		</Transition>

		<div v-if="sampleStore.sampleCards.length >= 2">
			<h2>Select files you want to merge</h2>
			<button class="btn" @click="sampleStore.downloadAll">Download All</button>
			<button :disabled="sampleStore.samples.length < 2" class="btn" @click="sampleStore.mergeAll">
				Merge All
			</button>
			<button
				:disabled="sampleStore.selectedSampleCards.length < 2"
				class="btn"
				@click="sampleStore.mergeSelected"
			>
				Merge selected
			</button>
			<button class="btn" @click="sampleStore.deleteAllFiles">Remove samples</button>
		</div>

		<Transition>
			<div ref="filesTemplateRef" class="samples" v-show="sampleStore.sampleCards.length">
				<SampleCard
					v-for="fileCard in sampleStore.sampleCards"
					v-bind="fileCard"
					@delete="sampleStore.deleteFile"
					v-model:selected="fileCard.selected"
					v-model:minimumCardPrice="fileCard.minimumCardPrice"
					@update:league="league => sampleStore.replaceFileCard(league, fileCard)"
				/>
			</div>
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

.samples {
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
