<script setup lang="ts">
import TabBadgeGroup from './TabBadgeGroup.vue';
import { tabBadgeProps as props } from './dummy';
</script>

<template>
	<Story>
		<TabBadgeGroup :stashes="props.stashes" :league="props.league" />
	</Story>
</template>

<style scoped></style>
