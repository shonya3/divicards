<script setup lang="ts">
import {
	SampleCardElement,
	Props,
	Events,
	DeleteThisSampleEvent,
} from '@divicards/wc/src/wc/e-sample-card/e-sample-card';
import '@divicards/wc/src/wc/e-sample-card/e-sample-card';
import { DivinationCardsSample } from '@divicards/shared/types';
import { League } from '@divicards/shared/types';
import { LeagueChangeEvent } from '@divicards/wc/src/wc/events/change/league';
const props = defineProps<Props>();
const emit = defineEmits<{
	'update:selected': [SampleCardElement['selected']];
	'update:league': [league: League];
	'update:minimumCardPrice': [SampleCardElement['minimumCardPrice']];
	delete: [uuid: string];
	'google-sheets-clicked': [DivinationCardsSample, League];
	'save-to-file-clicked': [DivinationCardsSample, League, string];
}>();

const onUpdLeague = (e: LeagueChangeEvent) => {
	emit('update:league', e.league);
};

const onUpdSelected = (e: CustomEvent<Events['upd:selected']>) => {
	emit('update:selected', e.detail);
};

const onUpdPrice = (e: CustomEvent<Events['upd:minimumCardPrice']>) => {
	emit('update:minimumCardPrice', e.detail);
};

const emit_delete_this_sample = ({ uuid }: DeleteThisSampleEvent) => {
	emit('delete', uuid);
};

const onGoogleSheetsClicked = (e: CustomEvent<Events['google-sheets-clicked']>) => {
	emit('google-sheets-clicked', e.detail.sample, e.detail.league);
};

const onSaveToFileClicked = (e: CustomEvent<Events['save-to-file-clicked']>) => {
	emit('save-to-file-clicked', e.detail.sample, e.detail.league, e.detail.filename);
};
</script>

<template>
	<e-sample-card
		v-bind="props"
		@google-sheets-clicked="onGoogleSheetsClicked"
		@save-to-file-clicked="onSaveToFileClicked"
		@change:league="onUpdLeague"
		@upd:selected="onUpdSelected"
		@upd:minimumCardPrice="onUpdPrice"
		@sample__delete="emit_delete_this_sample"
	></e-sample-card>
</template>
