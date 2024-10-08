<script setup lang="ts">
import { ref, Ref, shallowRef } from 'vue';
import { StashLoader } from './StashLoader';
import { command } from './command';
import { toast } from './toast';
import { isTauriError } from './error';
import { League } from '@divicards/shared/types';
import { downloadText } from '@divicards/shared/lib';
import { useSampleStore } from './stores/sample';
import { useGoogleAuthStore } from './stores/googleAuth';
import { useAuthStore } from './stores/auth';
import { useAutoAnimate } from './composables/useAutoAnimate';
import SampleCard from './components/SampleCard.vue';
import StashesView from './components/StashesView.vue';
import '@shoelace-style/shoelace/dist/components/copy-button/copy-button.js';
import { BasePopupElement } from '@divicards/wc/src/wc/e-base-popup';
import UpdateChangelog from './components/UpdateChangelog.vue';
import NativeBrowserLink from './components/NativeBrowserLink.vue';
import { useAppVersion } from './composables/useAppVersion';
import GeneralTabWithItems from './components/GeneralTabWithItems.vue';
import { useTauriUpdater } from './composables/useTauriUpdater';
import { TabWithItems } from 'poe-custom-elements/types.js';
import { SampleCardElement, SubmitExportSampleEvent } from '@divicards/wc/src/wc/e-sample-card/e-sample-card';

const dropZoneRef = shallowRef<HTMLElement | null>(null);
const sampleStore = useSampleStore();
const authStore = useAuthStore();
const googleAuthStore = useGoogleAuthStore();
const stashVisible = ref(false);
const { releaseUrl, tag } = useAppVersion();
const { update, installAndRelaunch } = useTauriUpdater();
const stashLoader = new StashLoader();
const tabsWithItems: Ref<TabWithItems[]> = ref<TabWithItems[]>([]);
const changelogPopupRef = ref<BasePopupElement | null>(null);
const samplesContainerRef = ref<HTMLElement | null>(null) as Ref<HTMLElement | null>;
useAutoAnimate(samplesContainerRef);

const openStashWindow = async () => {
	if (!authStore.loggedIn) {
		await authStore.login();
	}
	stashVisible.value = true;
};

async function export_sample({
	spreadsheetId,
	sheetTitle,
	preferences: table_preferences,
	sample,
	league,
	export_sample_to,
	filename,
	target,
}: SubmitExportSampleEvent) {
	const preferences = { ...table_preferences, columns: Array.from(table_preferences.columns) };

	switch (export_sample_to) {
		case 'file': {
			const csv = await command('sample_into_csv', { sample, preferences });
			downloadText(filename, csv);
			(target as SampleCardElement).form_popup.open = false;
			break;
		}

		case 'sheets': {
			if (!googleAuthStore.loggedIn) {
				await googleAuthStore.login();
			}
			try {
				const url = await command('new_sheet_with_sample', {
					spreadsheetId: spreadsheetId ?? '',
					title: sheetTitle ?? '',
					sample,
					preferences,
					league,
				});
				toast('success', 'New sheet created successfully');
				(target as SampleCardElement).form_popup.open = false;
				command('open_url', { url });
			} catch (err) {
				if (isTauriError(err)) {
					// TODO
					// exportSample.sheetsError = err.message;
				} else {
					console.log(err);
					(target as SampleCardElement).form_popup.open = false;
					throw err;
				}
			}
			break;
		}
	}
}

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
		@drop.prevent="
			(e: DragEvent) => {
				sampleStore.addFromDragAndDrop(e);
				dropZoneRef?.classList.remove('drop-zone--active');
			}
		"
		@dragenter="
			(e: DragEvent) => {
				e.preventDefault();
				dropZoneRef?.classList.add('drop-zone--active');
			}
		"
		@dragover="
			(e: DragEvent) => {
				e.preventDefault();
				dropZoneRef?.classList.add('drop-zone--active');
			}
		"
		@dragleave="
			(e: DragEvent) => {
				e.preventDefault();
				dropZoneRef?.classList.remove('drop-zone--active');
			}
		"
		class="drop-zone"
	>
		<header class="header">
			<e-drop-files-message></e-drop-files-message>
			<sl-button v-if="!stashVisible" @click="openStashWindow()">Load from stash</sl-button>
			<div class="header__right">
				<e-google-auth
					v-if="googleAuthStore.loggedIn"
					@login="googleAuthStore.login"
					@logout="googleAuthStore.logout"
					:name="googleAuthStore.name"
					:picture="googleAuthStore.picture"
					:loggedIn="googleAuthStore.loggedIn"
				></e-google-auth>
				<e-poe-auth
					v-if="authStore.loggedIn"
					@login="authStore.login"
					@logout="authStore.logout"
					:name="authStore.name"
					:loggedIn="authStore.loggedIn"
				></e-poe-auth>
				<sl-button
					variant="success"
					v-if="update && update.available"
					@click="() => changelogPopupRef?.showModal()"
					>Update is ready</sl-button
				>
				<e-theme-toggle></e-theme-toggle>
			</div>
		</header>
		<e-base-popup v-if="update" ref="changelogPopupRef">
			<UpdateChangelog @update-clicked="installAndRelaunch" :version="update.version" />
		</e-base-popup>
		<div v-show="authStore.loggedIn && stashVisible">
			<StashesView
				:stashLoader="stashLoader"
				@sample-from-tab="sampleStore.addSample"
				@stashtab-fetched="onTabWithItemsLoaded"
				@close="stashVisible = false"
				@extract-cards="extractCards"
			/>
		</div>
		<Transition>
			<SampleCard
				v-if="sampleStore.merged"
				v-bind="sampleStore.merged"
				@delete="sampleStore.deleteMerged"
				@update:minimumCardPrice="price => sampleStore.merged && (sampleStore.merged.minimumCardPrice = price)"
				@update:league="sampleStore.replaceMerged"
				@submit-sample="export_sample"
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
					@delete="sampleStore.deleteFile"
					v-model:selected="fileCard.selected"
					v-model:minimumCardPrice="fileCard.minimumCardPrice"
					@update:league="league => sampleStore.replaceFileCard(league, fileCard)"
					@submit-sample="export_sample"
				/>
			</div>
		</Transition>
	</div>
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
