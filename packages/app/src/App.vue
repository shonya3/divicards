<script setup lang="ts">
import { ref, Ref, shallowRef } from 'vue';
import { StashLoader } from './StashLoader';
import { command } from './command';
import { toast } from './toast';
import { isTauriError } from './error';
import { League } from '@divicards/shared/types';
import { DivinationCardsSample } from '@divicards/shared/types';
import { downloadText } from '@divicards/shared/lib';
import { useSampleStore } from './stores/sample';
import { useExportSampleStore } from './stores/exportSample';
import { useGoogleAuthStore } from './stores/googleAuth';
import { useAuthStore } from './stores/auth';
import { useTablePreferencesStore } from './stores/tablePreferences';
import { useAutoAnimate } from './composables/useAutoAnimate';
import SampleCard from './components/SampleCard.vue';
import StashesView from './components/StashesView.vue';
import FormExportSample from './components/FormExportSample.vue';
import { Props as FormExportProps } from '@divicards/wc/src/wc/form-export-sample/form-export-sample';
import '@shoelace-style/shoelace/dist/components/copy-button/copy-button.js';
import { BasePopupElement } from '@divicards/wc/src/wc/base-popup';
import UpdateChangelog from './components/UpdateChangelog.vue';
import NativeBrowserLink from './components/NativeBrowserLink.vue';
import { useAppVersion } from './composables/useAppVersion';
import { TabWithItems } from '@divicards/shared/poe.types';
import GeneralTabWithItems from './components/GeneralTabWithItems.vue';
import { useTauriUpdater } from './composables/useTauriUpdater';

const dropZoneRef = shallowRef<HTMLElement | null>(null);
const sampleStore = useSampleStore();
const authStore = useAuthStore();
const googleAuthStore = useGoogleAuthStore();
const exportSample = useExportSampleStore();
const tablePreferences = useTablePreferencesStore();
const stashVisible = ref(false);
const { releaseUrl, tag } = useAppVersion();
const { update, installAndRelaunch } = useTauriUpdater();
const stashLoader = new StashLoader();
const tabsWithItems: Ref<TabWithItems[]> = ref<TabWithItems[]>([]);
const changelogPopupRef = ref<BasePopupElement | null>(null);
const formPopupExportRef = ref<BasePopupElement | null>(null);
const samplesContainerRef = ref<HTMLElement | null>(null) as Ref<HTMLElement | null>;
useAutoAnimate(samplesContainerRef);

const openStashWindow = async () => {
	if (!authStore.loggedIn) {
		await authStore.login();
	}
	stashVisible.value = true;
};
const onSaveToFileClicked = (sample: DivinationCardsSample, league: League, filename: string) => {
	const name = filename.includes('.') ? filename : `${filename}.csv`;
	if (!formPopupExportRef.value) return;
	exportSample.to = 'file';
	exportSample.filename = name;
	exportSample.sample = sample;
	exportSample.league = league;
	formPopupExportRef.value.showModal();
};
const onGoogleSheetsClicked = (sample: DivinationCardsSample, league: League) => {
	if (!formPopupExportRef.value) return;
	exportSample.to = 'sheets';
	exportSample.sample = sample;
	exportSample.league = league;
	formPopupExportRef.value.showModal();
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
	const sample = exportSample.sample;
	const league = exportSample.league;
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
	if (exportSample.to === 'sheets') {
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
			formPopupExportRef.value?.close();
			command('open_url', { url });
			tablePreferences.rememberSpreadsheetId(spreadsheetId);
		} catch (err) {
			if (isTauriError(err)) {
				exportSample.sheetsError = err.message;
			} else {
				console.log(err);
				formPopupExportRef.value?.close();
				throw err;
			}
		}
	} else if (exportSample.to === 'file') {
		const csv = await command('sample_into_csv', { sample, preferences });
		downloadText(exportSample.filename, csv);
		formPopupExportRef.value?.close();
	}
};
const onTabWithItemsLoaded = (name: string, tab: TabWithItems, league: League) => {
	tab.items.sort((a, b) => (b.stackSize ?? 0) - (a.stackSize ?? 0));
	tabsWithItems.value.push(tab);
};

const extractCards = async (tab: TabWithItems, league: League) => {
	const sample = await command('extract_cards', { tab, league });
	sampleStore.addSample(tab.name, sample, league);
	toast('success', 'Cards successfully extracted');
};
</script>

<template>
	<div
		ref="dropZoneRef"
		@drop.prevent="(e: DragEvent) => {
            sampleStore.addFromDragAndDrop(e);
            dropZoneRef?.classList.remove('drop-zone--active')
        }"
		@dragenter="(e: DragEvent) => {
            e.preventDefault();
            dropZoneRef?.classList.add('drop-zone--active');
        }"
		@dragover="(e: DragEvent) => {
            e.preventDefault();
            dropZoneRef?.classList.add('drop-zone--active');
        }"
		@dragleave="(e: DragEvent) => {
            e.preventDefault();
            dropZoneRef?.classList.remove('drop-zone--active');
        }"
		class="drop-zone"
	>
		<header class="header">
			<wc-drop-files-message></wc-drop-files-message>
			<sl-button v-if="!stashVisible" @click="openStashWindow()">Load from stash</sl-button>
			<div class="header__right">
				<wc-google-auth
					v-if="googleAuthStore.loggedIn"
					@login="googleAuthStore.login"
					@logout="googleAuthStore.logout"
					:name="googleAuthStore.name"
					:picture="googleAuthStore.picture"
					:loggedIn="googleAuthStore.loggedIn"
				></wc-google-auth>
				<wc-poe-auth
					v-if="authStore.loggedIn"
					@login="authStore.login"
					@logout="authStore.logout"
					:name="authStore.name"
					:loggedIn="authStore.loggedIn"
				></wc-poe-auth>
				<sl-button
					variant="success"
					v-if="update && update.available"
					@click="() => changelogPopupRef?.showModal()"
					>Update is ready</sl-button
				>
				<theme-toggle></theme-toggle>
			</div>
		</header>
		<wc-base-popup v-if="update" ref="changelogPopupRef">
			<UpdateChangelog @update-clicked="installAndRelaunch" :version="update.version" />
		</wc-base-popup>
		<div v-show="authStore.loggedIn && stashVisible">
			<StashesView
				:stashLoader="stashLoader"
				@sample-from-tab="sampleStore.addSample"
				@tab-with-items-loaded="onTabWithItemsLoaded"
				@close="stashVisible = false"
				@extract-cards="extractCards"
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
			<h3>Select samples you want to merge</h3>
			<div class="sample-buttons">
				<sl-button :disabled="sampleStore.samples.length < 2" @click="sampleStore.mergeAll">
					Merge All
				</sl-button>
				<sl-button :disabled="sampleStore.selectedSampleCards.length < 2" @click="sampleStore.mergeSelected">
					Merge selected
				</sl-button>
				<sl-button @click="sampleStore.deleteAllFiles">Remove samples</sl-button>
			</div>
		</div>
		<ul class="general-tabs" v-for="tab in tabsWithItems">
			<li>
				<GeneralTabWithItems
					@close="
						() => {
							tabsWithItems = tabsWithItems.filter(({ id }) => id !== tab.id);
						}
					"
					:tab="tab"
				/>
			</li>
		</ul>
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
		<FormExportSample
			:error="exportSample.sheetsError"
			:to="exportSample.to"
			v-model:columns="tablePreferences.columns"
			v-model:order="tablePreferences.order"
			v-model:orderedBy="tablePreferences.orderedBy"
			v-model:cardsMustHaveAmount="tablePreferences.cardsMustHaveAmount"
			v-model:sheetTitle="tablePreferences.sheetTitle"
			v-model:minPrice="tablePreferences.minPrice"
			:spreadsheetId="tablePreferences.spreadsheetId"
			@submit="onSubmit"
		></FormExportSample>
	</wc-base-popup>
	<div class="version">
		<NativeBrowserLink :href="releaseUrl">{{ tag }}</NativeBrowserLink>
	</div>
</template>

<style scoped>
.header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	margin-bottom: 1.2rem;
}
.header__right {
	display: flex;
	gap: 1rem;
	align-items: center;
}

.v-enter-active,
.v-leave-active {
	transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
	opacity: 0;
}

.drop-zone {
	height: 100vh;
	position: relative;
	padding: 1rem;
	min-width: 800px;

	display: flex;
	flex-direction: column;
	gap: 0.8rem;
}
.drop-zone--active {
	background-color: red;
	background-color: var(--sl-color-cyan-400);
}

.general-tabs {
	list-style: none;
}

.samples {
	display: flex;
	flex-wrap: wrap;
	gap: 2rem;
}

.sample-buttons {
	margin-top: 0.4rem;
	display: flex;
	gap: 0.2rem;
}

.version {
	position: fixed;
	bottom: 0;
	right: 0;
	padding: 0.3rem;
	padding-right: 1rem;
}
</style>
