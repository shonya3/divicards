<script setup lang="ts">
import {
	SampleCardElement,
	Props,
	DeleteThisSampleEvent,
	SelectedChangeEvent,
	GoogleSheetsClickEvent,
	SaveToFileClickEvent,
	MinimumCardsPriceChangeEvent,
} from '@divicards/wc/src/wc/e-sample-card/e-sample-card';
import '@divicards/wc/src/wc/e-sample-card/e-sample-card';
import { DivinationCardsSample } from '@divicards/shared/types';
import { League } from '@divicards/shared/types';
import { LeagueChangeEvent } from '@divicards/wc/src/wc/events/change/league';
const props = defineProps<Props>();
const emit = defineEmits<{
	'update:selected': [selected: SampleCardElement['selected']];
	'update:league': [league: League];
	'update:minimumCardPrice': [minimum_card_price: number];
	delete: [uuid: string];
	'google-sheets-clicked': [sample: DivinationCardsSample, league: League];
	'save-to-file-clicked': [sample: DivinationCardsSample, league: League, filename: string];
}>();

const emit_change_league = (e: LeagueChangeEvent) => {
	emit('update:league', e.league);
};

const emit_change_selected = ({ selected }: SelectedChangeEvent) => {
	emit('update:selected', selected);
};

const emit_change_minimum_card_price = ({ minimum_card_price }: MinimumCardsPriceChangeEvent) => {
	emit('update:minimumCardPrice', minimum_card_price);
};

const emit_delete_this_sample = ({ uuid }: DeleteThisSampleEvent) => {
	emit('delete', uuid);
};

const emit_google_sheets_click = ({ sample, league }: GoogleSheetsClickEvent) => {
	emit('google-sheets-clicked', sample, league);
};

const emit_save_to_file_click = ({ sample, league, filename }: SaveToFileClickEvent) => {
	emit('save-to-file-clicked', sample, league, filename);
};
</script>

<template>
	<e-sample-card
		v-bind="props"
		@sample__google-sheets-click="emit_google_sheets_click"
		@sample__save-to-file-click="emit_save_to_file_click"
		@change:league="emit_change_league"
		@sample__change:selected="emit_change_selected"
		@sample__change:minimum_card_price="emit_change_minimum_card_price"
		@sample__delete="emit_delete_this_sample"
	></e-sample-card>
</template>
