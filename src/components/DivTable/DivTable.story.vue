<script setup lang="ts">
import { ref } from 'vue';
import DivTable, { DivTableProps } from './DivTable.vue';
import { divTableProps } from './dummy';

const props = ref<DivTableProps>(divTableProps);
</script>

<template>
	<Story>
		<DivTable v-bind="props" />
	</Story>
</template>

<style scoped></style>
