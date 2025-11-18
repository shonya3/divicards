<script setup lang="ts">
import { computed, ref, Ref, shallowRef, nextTick } from 'vue';
import { StashLoader } from './StashLoader';
import { command } from './command';
import { toast } from './toast';
import { isTauriError, handleError } from './error';
import { isTradeLeague, Column, League, Order } from '@divicards/shared/types.js';
import { downloadText, ACTIVE_LEAGUE } from '@divicards/shared/lib.js';
import { useSampleStore } from './stores/sample';
import { useGoogleAuthStore } from './stores/googleAuth';
import { useAuthStore } from './stores/auth';
import { useAutoAnimate } from './composables/useAutoAnimate';
import '@shoelace-style/shoelace/dist/components/copy-button/copy-button.js';
import { BasePopupElement } from '@divicards/wc/e-base-popup.js';
import UpdateChangelog from './components/UpdateChangelog.vue';
import NativeBrowserLink from './components/NativeBrowserLink.vue';
import { useAppVersion } from './composables/useAppVersion';
import GeneralTabWithItems from './components/GeneralTabWithItems.vue';
import { useTauriUpdater } from './composables/useTauriUpdater';
import { TabWithItems } from 'poe-custom-elements/types.js';
import { SampleCardElement } from '@divicards/wc/e-sample-card/e-sample-card.js';
import { webviewWindow } from '@tauri-apps/api';

import '@divicards/wc/e-theme-toggle/e-theme-toggle.js';
import '@divicards/wc/e-sample-card/e-sample-card.js';
import '@divicards/wc/stashes/e-stashes-view.js';
import '@divicards/wc/e-poe-auth/e-poe-auth.js';
import '@divicards/wc/e-drop-files-message.js';
import '@divicards/wc/e-import-file-tip.js';
import '@divicards/wc/e-league-select';
import '@divicards/wc/stashes/poe-general-priced-list.js';

import { SubmitExportSampleEvent } from '@divicards/wc/e-sample-card/events.js';
import { ExtractCardsEvent, StashtabFetchedEvent } from '@divicards/wc/stashes/events.js';
import { ChangeThemeEvent } from '@divicards/wc/e-theme-toggle/events.js';

const dropZoneRef = shallowRef<HTMLElement | null>(null);
const sampleStore = useSampleStore();
const authStore = useAuthStore();
const googleAuthStore = useGoogleAuthStore();
const stashVisible = ref(false);
const shouldShowImportActions = computed(() => !stashVisible.value || !authStore.loggedIn);
const { releaseUrl, tag } = useAppVersion();
const { update, installAndRelaunch } = useTauriUpdater();
const stashLoader = new StashLoader();
const league = ref<League>(ACTIVE_LEAGUE as League);
const tabsWithItems: Ref<TabWithItems[]> = ref<TabWithItems[]>([]);
const selectedIds = ref<string[]>([]);
const aggregatedTab = computed<TabWithItems | null>(() => {
    const map = new Map<string, TabWithItems>();
    for (const t of tabsWithItems.value) map.set(t.id, t);
    const ids = selectedIds.value.length ? selectedIds.value : Array.from(map.keys());
    const items: TabWithItems['items'] = [];
    for (const id of ids) {
        const t = map.get(id);
        if (t) items.push(...t.items);
    }
    if (!items.length) return null;
    return { id: 'aggregate', name: 'Aggregate', type: 'NormalStash', index: 0, items } as TabWithItems;
});
const availableTabs = ref<{ id: string; name: string; type: string }[]>([]);
const selectedTabId = ref<string | null>(null);
const settingsPopupRef = ref<BasePopupElement | null>(null);
const extractingSelected = ref(false);
const hasExportableSample = computed(() => !!sampleStore.merged || sampleStore.sampleCards.length > 0);
const exportableLeague = computed<League | null>(() => {
    if (sampleStore.merged) return sampleStore.merged.league as League;
    if (sampleStore.sampleCards.length > 0) return sampleStore.sampleCards[0].league as League;
    return null;
});
const changelogPopupRef = ref<BasePopupElement | null>(null);
const samplesContainerRef = ref<HTMLElement | null>(null) as Ref<HTMLElement | null>;
useAutoAnimate(samplesContainerRef);
const gemCacheMinutes = ref<number>(15);
const stashesViewRef = shallowRef<HTMLElement | null>(null);
const bulkMode = ref<boolean>(false);

const bulkLoadStash = async () => {
    if (!authStore.loggedIn) {
        await authStore.login();
    }
    stashVisible.value = true;
    bulkMode.value = true;
    await nextTick();
    stashesViewRef.value?.dispatchEvent(new Event('stashes__bulk-load-all'));
};

async function quickExportToSheets() {
    if (!googleAuthStore.loggedIn) {
        await googleAuthStore.login();
    }
    const hasMerged = !!sampleStore.merged;
    const hasAny = sampleStore.sampleCards.length > 0;
    if (!hasMerged && !hasAny) {
        toast('warning', 'Add a sample first (drag CSV or load stash)');
        return;
    }
    let sample;
    let league: League;
    if (hasMerged) {
        sample = sampleStore.merged!.sample;
        league = sampleStore.merged!.league as League;
    } else {
        sample = sampleStore.sampleCards[0].sample;
        league = sampleStore.sampleCards[0].league as League;
    }
    let spreadsheetId = googleAuthStore.spreadsheetId;
    if (!spreadsheetId) {
        spreadsheetId = window.prompt('Enter Spreadsheet ID') ?? '';
        googleAuthStore.spreadsheetId = spreadsheetId;
    }
    const now = new Date();
    const title = `Divicards Export - ${now.toLocaleString()}`;
    const preferences = {
        columns: ['name', 'amount', 'weight', 'price', 'sum'] as Column[],
        orderedBy: 'amount' as Column,
        order: 'desc' as Order,
        cardsMustHaveAmount: true,
        minPrice: 0,
    };
    try {
        const url = await command('new_sheet_with_sample', {
            spreadsheetId,
            title,
            sample,
            preferences,
            league,
        });
        toast('success', 'Exported to Google Sheets');
        googleAuthStore.spreadsheetId = spreadsheetId;
        command('open_url', { url });
    } catch (err) {
        handleError(err);
    }
}

const openStashWindow = async () => {
    if (!authStore.loggedIn) {
        await authStore.login();
    }
    stashVisible.value = true;
    bulkMode.value = false;
    const tabs = await stashLoader.tabs(league.value);
    availableTabs.value = tabs.filter(t => t.type === 'DivinationCardStash');
    if (availableTabs.value.length && !selectedTabId.value) {
        selectedTabId.value = availableTabs.value[0].id;
    }
};


const extractSelectedTab = async () => {
    if (extractingSelected.value) return;
    if (!authStore.loggedIn) {
        await authStore.login();
    }
    if (!selectedTabId.value) {
        toast('warning', 'Select a Divination Card tab');
        return;
    }
    try {
        extractingSelected.value = true;
        const tab = availableTabs.value.find(t => t.id === selectedTabId.value);
        if (!tab) {
            toast('warning', 'Selected tab not found');
            extractingSelected.value = false;
            return;
        }
        const sample = await stashLoader.sampleFromTab(tab.id, league.value);
        await sampleStore.addSample(tab.name, sample, league.value);
        toast('success', `Extracted cards from ${tab.name}`);
    } catch (err) {
        console.log('Failed to extract selected tab', selectedTabId.value, err);
        toast('danger', 'Failed to extract selected tab');
    }
    extractingSelected.value = false;
};

function changeLeague(e: any) {
    const l = e.$league as League;
    if (isTradeLeague(l)) {
        league.value = l;
    }
}

async function export_sample({
    spreadsheetId,
    sheetTitle,
    preferences: table_preferences,
    $sample,
    $league,
    export_sample_to,
    $filename,
    target,
}: SubmitExportSampleEvent) {
    const preferences = {
        ...table_preferences,
        columns: (() => {
            const cols = Array.from(table_preferences.columns);
            if (!cols.includes('name')) cols.unshift('name');
            if (!cols.includes('amount')) cols.splice(1, 0, 'amount');
            return cols;
        })(),
        cardsMustHaveAmount: table_preferences.cardsMustHaveAmount ?? true,
    };

    switch (export_sample_to) {
        case 'file': {
            const csv = await command('sample_into_csv', { sample: $sample, preferences });
            downloadText($filename, csv);
            (target as SampleCardElement).form_popup.open = false;
            break;
        }

        case 'sheets': {
            if (!googleAuthStore.loggedIn) {
                await googleAuthStore.login();
            }
            const now = new Date();
            const defaultTitle = `Divicards Export - ${now.toLocaleString()}`;
            const id = (spreadsheetId || googleAuthStore.spreadsheetId).trim();
            const finalTitle = sheetTitle || defaultTitle;
            if (!id || !finalTitle) {
                toast('warning', 'Enter Spreadsheet ID and Sheet Title');
                return;
            }
            try {
                const url = await command('new_sheet_with_sample', {
                    spreadsheetId: id,
                    title: finalTitle,
                    sample: $sample,
                    preferences,
                    league: $league,
                });
                toast('success', 'New sheet created successfully');
                (target as SampleCardElement).form_popup.open = false;
                googleAuthStore.spreadsheetId = id;
                command('open_url', { url });
            } catch (err) {
                if (isTauriError(err)) {
                    toast('danger', err.message ?? 'Export failed');
                } else {
                    console.log(err);
                    (target as SampleCardElement).form_popup.open = false;
                    toast('danger', 'Export failed');
                }
            }
            break;
        }
    }
}

const handle_stashtab_fetched = (e: StashtabFetchedEvent) => {
	e.$stashtab.items.sort((a, b) => (b.stackSize ?? 0) - (a.stackSize ?? 0));
	tabsWithItems.value.push(e.$stashtab);
};

const handle_extract_cards = async (e: ExtractCardsEvent) => {
	const sample = await command('extract_cards', { tab: e.$tab, league: e.$league });
	sampleStore.addSample(e.$tab.name, sample, e.$league);
};

const handle_change_theme = (e: ChangeThemeEvent) => {
    const isTauri =
        (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ != null) ||
        (typeof navigator !== 'undefined' && navigator.userAgent.includes('Tauri')) ||
        (typeof import.meta !== 'undefined' && (import.meta as any).env && ((import.meta as any).env.TAURI_PLATFORM ?? (import.meta as any).env.TAURI));
    if (!isTauri) return;
    webviewWindow.WebviewWindow.getCurrent().setTheme(e.$theme);
};

// --Dragzone handlers
const isDragging = ref(false);
const handleDropZoneDragEnter = (event: DragEvent) => {
	event.preventDefault();
	// Only activate if files are being dragged
	if (event.dataTransfer && Array.from(event.dataTransfer.types).includes('Files')) {
		dropZoneRef.value?.classList.add('drop-zone--active');
		isDragging.value = true;
	}
};

const handleDropZoneDragOver = (event: DragEvent) => {
	event.preventDefault(); // Necessary to allow dropping
};

const handleDropZoneDragLeave = (event: DragEvent) => {
	event.preventDefault();
	const dropZoneEl = dropZoneRef.value;
	// Check if the mouse is truly leaving the dropZoneEl, not just moving to a child.
	if (dropZoneEl && (event.relatedTarget === null || !dropZoneEl.contains(event.relatedTarget as Node))) {
		dropZoneEl.classList.remove('drop-zone--active');
		isDragging.value = false;
	}
};

const handleDropZoneDrop = (event: DragEvent) => {
	event.preventDefault();
	dropZoneRef.value?.classList.remove('drop-zone--active');
	isDragging.value = false;
	sampleStore.addFromDragAndDrop(event);
};
</script>

<template>
	<div
		ref="dropZoneRef"
		@drop.prevent="handleDropZoneDrop"
		@dragenter.prevent="handleDropZoneDragEnter"
		@dragover.prevent="handleDropZoneDragOver"
		@dragleave.prevent="handleDropZoneDragLeave"
		class="drop-zone"
	>
		<div v-if="isDragging" class="drop-overlay-message">
			<e-drop-files-message></e-drop-files-message>
		</div>
        <header class="toolbar">
            <div class="toolbar__left">
                <e-league-select with-private-league-input :league="league" @change:league="changeLeague"></e-league-select>
                <sl-button variant="primary" @click="openStashWindow()"
                    >Load from Stash</sl-button
                >
                <sl-button variant="default" @click="bulkLoadStash">Bulk load Stash</sl-button>
                <e-import-file-tip v-if="!isDragging && shouldShowImportActions"></e-import-file-tip>
            </div>
            <div class="toolbar__right">
                <sl-button v-if="!googleAuthStore.loggedIn" @click="googleAuthStore.login" variant="default"
                    >Sign in with Google</sl-button
                >
                <sl-button
                    v-if="googleAuthStore.loggedIn"
                    :disabled="!hasExportableSample"
                    @click="quickExportToSheets"
                    variant="primary"
                    >Export to Google Sheets</sl-button
                >
                <sl-input
                    v-if="googleAuthStore.loggedIn"
                    class="spreadsheet-id-input"
                    placeholder="Saved Spreadsheet ID"
                    :value="googleAuthStore.spreadsheetId"
                    @sl-input="(e: any) => (googleAuthStore.spreadsheetId = e.target.value)"
                ></sl-input>
                <sl-button v-if="googleAuthStore.loggedIn" variant="default" @click="() => settingsPopupRef?.showModal()"
                    >Settings</sl-button
                >
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
                    @poe-auth__login="authStore.login"
                    @poe-auth__logout="authStore.logout"
                    :auth="{
                        loggedIn: authStore.loggedIn,
                        username: authStore.name,
                    }"
                ></e-poe-auth>
                <sl-button
                    variant="success"
                    v-if="update && update.available"
                    @click="() => changelogPopupRef?.showModal()"
                    >Update is ready</sl-button
                >
                <e-theme-toggle @theme-toggle__change:theme="handle_change_theme"></e-theme-toggle>
            </div>
        </header>
        <sl-divider></sl-divider>

        <e-base-popup v-if="update" ref="changelogPopupRef">
            <UpdateChangelog @update-clicked="installAndRelaunch" :version="update.version" />
        </e-base-popup>
        <e-base-popup ref="settingsPopupRef">
            <div style="display:flex;flex-direction:column;gap:0.8rem;min-width:400px">
                <h3>Settings</h3>
                <sl-input
                    placeholder="Spreadsheet ID"
                    :value="googleAuthStore.spreadsheetId"
                    @sl-input="(e: any) => (googleAuthStore.spreadsheetId = e.target.value)"
                ></sl-input>
                <sl-input
                    placeholder="Gem pricing cache minutes"
                    type="number"
                    :value="gemCacheMinutes"
                    @sl-input="(e: any) => (gemCacheMinutes = Number(e.target.value))"
                ></sl-input>
                <sl-button
                    variant="primary"
                    @click="async () => { await command('set_gem_prices_cache_ttl_minutes', { minutes: Number(gemCacheMinutes) }); settingsPopupRef?.close(); }"
                >Save</sl-button>
            </div>
        </e-base-popup>
        <e-stashes-view
            v-show="authStore.loggedIn && stashVisible"
            :league="league"
            :stashLoader="stashLoader"
            ref="stashesViewRef"
            @stashes__sample-from-stashtab="e => sampleStore.addSample(e.$stashtab_name, e.$sample, e.$league)"
            @stashes__stashtab-fetched="handle_stashtab_fetched"
            @stashes__close="stashVisible = false"
            @stashes__extract-cards="handle_extract_cards"
            @change:selected_tabs="e => (selectedIds = Array.from(e.$selected_tabs.keys()))"
        ></e-stashes-view>
		<Transition>
			<div>
				<e-sample-card
					v-if="!bulkMode && sampleStore.merged"
					v-bind="sampleStore.merged"
					@sample__delete="sampleStore.deleteMerged"
					@sample__change:minimum_card_price="
						e => {
							if (!sampleStore.merged) return;
							sampleStore.merged.minimumCardPrice = e.$minimum_card_price;
						}
					"
					@change:league="
						e => {
							if (!sampleStore.merged || !isTradeLeague(e.$league)) return;
							sampleStore.merged.league = e.$league;
							sampleStore.replaceMerged(e.$league);
						}
					"
					@sample__submit-export-sample="export_sample"
				></e-sample-card>
			</div>
		</Transition>
		<div v-if="!bulkMode && sampleStore.sampleCards.length >= 2">
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
        <div v-if="aggregatedTab">
            <poe-general-priced-list aggregate :league="league" :stashLoader="stashLoader" :tab="aggregatedTab as any"></poe-general-priced-list>
        </div>
		<Transition>
			<div ref="filesTemplateRef" class="samples" v-show="!bulkMode && sampleStore.sampleCards.length">
				<e-sample-card
					v-for="fileCard in sampleStore.sampleCards"
					v-bind="fileCard"
					@sample__delete="e => sampleStore.deleteFile(e.$uuid)"
					@sample__change:selected="e => (fileCard.selected = e.$selected)"
					@sample__change:minimum_card_price="e => (fileCard.minimumCardPrice = e.$minimum_card_price)"
					@change:league="
						e => {
							if (!isTradeLeague(e.$league)) return;
							fileCard.league = e.$league;
							sampleStore.replaceFileCard(e.$league, fileCard);
						}
					"
					@sample__change:filename="
						e => {
							fileCard.filename = e.$filename;
						}
					"
					@sample__submit-export-sample="export_sample"
				></e-sample-card>
			</div>
		</Transition>
	</div>
	<div class="version">
		<NativeBrowserLink :href="releaseUrl">{{ tag }}</NativeBrowserLink>
	</div>
</template>

<style scoped>
.import-actions {
	display: flex;
	gap: 1rem;
	align-items: center;
}

.drop-overlay-message {
	position: absolute;
	top: 0;
	left: 0;
	right: 0;
	bottom: 0;
	display: flex;
	justify-content: center;
	align-items: center;
	z-index: 10; /* Ensure it's above other content but below popups/modals */
	pointer-events: none; /* Allows drag events to pass through to the drop-zone itself */
}

.toolbar {
    display: grid;
    grid-template-columns: 1fr auto;
    align-items: center;
    gap: 0.8rem;
}
.toolbar__left,
.toolbar__right {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
    align-items: center;
}
.spreadsheet-id-input {
    min-width: 360px;
    flex: 1 1 360px;
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
