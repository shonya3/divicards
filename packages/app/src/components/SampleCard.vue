<script setup lang="ts">
import {
	SampleCardElement,
	Props,
	DeleteThisSampleEvent,
	SelectedChangeEvent,
	MinimumCardsPriceChangeEvent,
	SubmitExportSampleEvent,
} from '@divicards/wc/e-sample-card/e-sample-card.js';
import '@divicards/wc/e-sample-card/e-sample-card';
import { League } from '@divicards/shared/types.js';
import { LeagueChangeEvent } from '@divicards/wc/events/change/league.js';
const props = defineProps<Props>();
const emit = defineEmits<{
	'update:selected': [selected: SampleCardElement['selected']];
	'update:league': [league: League];
	'update:minimumCardPrice': [minimum_card_price: number];
	delete: [uuid: string];
	'submit-sample': [event: SubmitExportSampleEvent];
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

const emit_submit_sample = (e: SubmitExportSampleEvent) => {
	emit('submit-sample', e);
};
</script>

<template>
	<e-sample-card
		v-bind="props"
		@change:league="emit_change_league"
		@sample__change:selected="emit_change_selected"
		@sample__change:minimum_card_price="emit_change_minimum_card_price"
		@sample__delete="emit_delete_this_sample"
		@sample__submit-export-sample="emit_submit_sample"
	></e-sample-card>
</template>
