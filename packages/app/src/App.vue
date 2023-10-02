<script setup lang="ts">
import { ref } from 'vue';

import { useSampleStore } from './stores/sample';
import { useAuthStore } from './stores/auth';
import { useAutoAnimate } from './composables/useAutoAnimate';

import SampleCard from './components/SampleCard.vue';
import StashesView from './components/StashesView.vue';
import FormSampleExport from './components/FormSampleExport.vue';
import { DropFilesMessageElement } from '@divicards/wc/src/wc/drop-files-message';
import { PoeAuthElement } from '@divicards/wc/src/wc/poe-auth';
import { StashLoader } from './StashLoader';
import { useGoogleAuthStore } from './stores/googleAuth';
import { GoogleAuthElement } from '../../wc/src/wc/google-auth/poe-auth';
import { command } from './command';
import { BasePopupElement } from '../../wc/src/wc/base-popup';
import { Props as FormExportProps } from '@divicards/wc/src/wc/form-export-sample/form-export-sample';
import { DivinationCardsSample } from '../../shared/types';
import { toast } from './toast';
import { usePreferences } from './composables/usePreferences';
import { isTauriError } from './error';
import { League } from '@divicards/shared/types';
import { downloadText } from '../../shared/lib';
import { useExportState } from './composables/useExportState';
BasePopupElement.define();
DropFilesMessageElement.define();
PoeAuthElement.define();
GoogleAuthElement.define();
const stashLoader = new StashLoader();

const { spreadsheet, columns, order, orderedBy, cardsMustHaveAmount, sheetTitle, minPrice } = usePreferences();
const exportState = useExportState();
const formPopupExportRef = ref<BasePopupElement | null>(null);

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

const onSaveToFileClicked = (sample: DivinationCardsSample, league: League, filename: string) => {
	if (!formPopupExportRef.value) return;
	exportState.to.value = 'file';
	exportState.filename.value = filename;
	exportState.sample.value = sample;
	exportState.league.value = league;
	formPopupExportRef.value.open();
};

const onGoogleSheetsClicked = (sample: DivinationCardsSample, league: League) => {
	if (!formPopupExportRef.value) return;
	exportState.to.value = 'sheets';
	exportState.sample.value = sample;
	exportState.league.value = league;
	formPopupExportRef.value.open();
};

const onSubmit = async ({
	spreadsheetId,
	sheetTitle,
	order,
	orderedBy,
	columns,
	cardsMustHaveAmount,
	minPrice,
}: FormExportProps) => {
	const sample = exportState.sample.value;
	const league = exportState.league.value;

	if (!sample) {
		throw new Error('No sample to sheets');
	}

	if (!league) {
		throw new Error('No league to sheets');
	}

	const preferences = {
		cardsMustHaveAmount,
		order,
		orderedBy,
		columns: Array.from(columns),
		minPrice,
	};

	if (exportState.to.value === 'sheets') {
		if (!googleAuthStore.loggedIn) {
			await googleAuthStore.login();
		}

		try {
			const url = await command('new_sheet_with_sample', {
				spreadsheetId,
				title: sheetTitle,
				sample,
				preferences,
				league,
			});

			toast('success', 'New sheet created successfully');
			formPopupExportRef.value?.hide();
			command('open_url', { url });

			// In the end,  save spreadshetId to LocalStorage
			spreadsheet.value = spreadsheetId;
			return;
		} catch (err) {
			if (isTauriError(err)) {
				exportState.sheetsError.value = err.message;
			} else {
				console.log(err);
				formPopupExportRef.value?.hide();
				throw err;
			}
		}
	} else if (exportState.to.value === 'file') {
		const csv = await command('sample_into_csv', { sample, preferences });
		downloadText(exportState.filename.value, csv);
		formPopupExportRef.value?.hide();
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
				@save-to-file-clicked="onSaveToFileClicked"
				@google-sheets-clicked="onGoogleSheetsClicked"
				@update:minimumCardPrice="price => sampleStore.merged && (sampleStore.merged.minimumCardPrice = price)"
				@update:league="sampleStore.replaceMerged"
			/>
		</Transition>

		<div v-if="sampleStore.sampleCards.length >= 2">
			<h2>Select samples you want to merge</h2>
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
					@save-to-file-clicked="onSaveToFileClicked"
					@google-sheets-clicked="onGoogleSheetsClicked"
					@delete="sampleStore.deleteFile"
					v-model:selected="fileCard.selected"
					v-model:minimumCardPrice="fileCard.minimumCardPrice"
					@update:league="league => sampleStore.replaceFileCard(league, fileCard)"
				/>
			</div>
		</Transition>
	</div>

	<wc-base-popup ref="formPopupExportRef">
		<FormSampleExport
			:error="exportState.sheetsError.value"
			:to="exportState.to.value"
			v-model:columns="columns"
			v-model:order="order"
			v-model:orderedBy="orderedBy"
			v-model:cardsMustHaveAmount="cardsMustHaveAmount"
			v-model:sheetTitle="sheetTitle"
			v-model:minPrice="minPrice"
			:spreadsheet-id="spreadsheet"
			@submit="onSubmit"
		></FormSampleExport>
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
