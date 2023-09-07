<script setup lang="ts">
import { ref } from 'vue';

import { useSampleStore } from './stores/sample';
import { useAuthStore } from './stores/auth';
import { useAutoAnimate } from './composables/useAutoAnimate';

import SampleCard from './components/SampleCard.vue';
import StashesView from './components/StashesView.vue';
import ToGoogleSheets from './components/ToGoogleSheets.vue';
import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';
import { StashLoader } from './StashLoader';
import { useGoogleAuthStore } from './stores/googleAuth';
import { GoogleAuthElement } from '../../wc/src/wc/google-auth/poe-auth';
import { command } from './command';
import { SheetsApi } from './sheets';
import { BasePopupElement } from '../../wc/src/wc/base-popup';
import { Props as SheetsProps } from '@divicards/wc/src/wc/to-google-sheets/to-google-sheets';
import { DivinationCardsSample } from '../../shared/types';
import { toast } from './toast';
import { SheetsError, isSheetsError } from './error';
import { useSheets } from './composables/useSheets';
BasePopupElement.define();
DropFilesMessageElement.define();
PoeAuthElement.define();
GoogleAuthElement.define();
const stashLoader = new StashLoader();
const sheetsApi = new SheetsApi();

const toSheetsSample = ref<DivinationCardsSample | null>(null);
const sheetsError = ref<null | SheetsError>(null);
const { spreadsheet, columns, order, orderedBy, cardsMustHaveAmount, sheetTitle } = useSheets();

const sheetsPopupRef = ref<BasePopupElement | null>(null);

const sampleStore = useSampleStore();
const authStore = useAuthStore();
const googleAuthStore = useGoogleAuthStore();

const stashVisible = ref(false);
const samplesContainerRef = ref<HTMLElement | null>(null);
useAutoAnimate(samplesContainerRef);

const openStashWindow = async () => {
	if (!authStore.loggedIn) {
		await authStore.login();
	}

	stashVisible.value = true;
};

const onGoogleSheetsClicked = (sample: DivinationCardsSample) => {
	if (!sheetsPopupRef.value) return;
	toSheetsSample.value = sample;
	sheetsPopupRef.value.open();
};

const onSheetsSubmit = async ({
	spreadsheetId,
	sheetTitle,
	order,
	orderedBy,
	columns,
	cardsMustHaveAmount,
}: SheetsProps) => {
	if (!toSheetsSample.value) {
		throw new Error('No sample to sheets');
	}

	if (!googleAuthStore.loggedIn) {
		await googleAuthStore.login();
	}

	try {
		const { url } = await sheetsApi.createSheetWithSample(
			spreadsheetId,
			sheetTitle,
			toSheetsSample.value,
			googleAuthStore.token,
			{ cardsMustHaveAmount, order, orderedBy, columns }
		);

		toast('success', 'New sheet created successfully');
		sheetsPopupRef.value?.hide();
		command('open_url', { url });

		// save spreadshetId to LocalStorage
		spreadsheet.value = spreadsheetId;
	} catch (err) {
		if (isSheetsError(err)) {
			sheetsError.value = err;
		} else throw err;
	}
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
			<wc-google-auth
				@login="googleAuthStore.login"
				@logout="googleAuthStore.logout"
				:name="googleAuthStore.name"
				:picture="googleAuthStore.picture"
				:loggedIn="googleAuthStore.loggedIn"
			></wc-google-auth>
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
				@google-sheets-clicked="onGoogleSheetsClicked(sampleStore.merged.sample)"
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
					@google-sheets-clicked="onGoogleSheetsClicked(fileCard.sample)"
					@delete="sampleStore.deleteFile"
					v-model:selected="fileCard.selected"
					v-model:minimumCardPrice="fileCard.minimumCardPrice"
					@update:league="league => sampleStore.replaceFileCard(league, fileCard)"
				/>
			</div>
		</Transition>
	</div>

	<wc-base-popup ref="sheetsPopupRef">
		<ToGoogleSheets
			:error="sheetsError"
			v-model:columns="columns"
			v-model:order="order"
			v-model:orderedBy="orderedBy"
			v-model:cardsMustHaveAmount="cardsMustHaveAmount"
			v-model:sheetTitle="sheetTitle"
			:spreadsheet-id="spreadsheet"
			@submit="onSheetsSubmit"
		></ToGoogleSheets>
	</wc-base-popup>
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
