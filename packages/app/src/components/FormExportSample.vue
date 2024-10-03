<script setup lang="ts">
import { Props } from '@divicards/wc/src/wc/e-sample-card/e-form-export-sample/e-form-export-sample';
import '@divicards/wc/src/wc/e-sample-card/e-form-export-sample/e-form-export-sample';
const props = withDefaults(defineProps<Props>(), { to: 'sheets', spreadsheetId: '', sheetTitle: '' });
const emit = defineEmits<{
	submit: [Props];
}>();

const handlers = {
	onSubmit(e: CustomEvent<Props>) {
		emit('submit', e.detail);
	},
};
</script>

<template>
	<e-form-export-sample v-bind="props" @submit="handlers.onSubmit"></e-form-export-sample>
</template>

<style scoped>
wc-form-export-sample {
	padding-inline: 0;
}
</style>
