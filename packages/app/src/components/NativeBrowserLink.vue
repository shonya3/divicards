<script setup lang="ts">
import { command } from '../command';

defineProps<{ href: string }>();
</script>

<template>
	<a @click.prevent="command('open_url', { url: href })" :href="href">
		<slot />
	</a>
</template>

<style scoped></style>
