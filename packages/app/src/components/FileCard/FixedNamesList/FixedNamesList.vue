<script setup lang="ts">
import BasePopup from '../../BasePopup.vue';
import ArrowRight from '../../icons/ArrowRight.vue';
import FixedIcon from './FixedIcon.vue';
import { ref } from 'vue';
import { FixedName } from '@divicards/shared/types';

const fixedNamesPopup = ref<typeof BasePopup | null>(null);

defineProps<{
	fixedNames: FixedName[];
}>();
</script>

<template>
	<FixedIcon v-if="fixedNames.length" @click="fixedNamesPopup?.open()" :width="24" :height="24" />
	<BasePopup ref="fixedNamesPopup">
		<div class="fixed-names">
			<h2>Automatically fixed typos</h2>
			<ul class="fixed-names-list">
				<li class="list-item" v-for="{ old, fixed } in fixedNames">
					<span class="input-name">{{ old }}</span>
					<ArrowRight />
					<span class="fixed-name">{{ fixed }}</span>
				</li>
			</ul>
		</div>
	</BasePopup>
</template>

<style scoped>
.fixed-names-list {
	margin-top: 2rem;
	/* max-width: 400px; */
}

.list-item {
	display: flex;
	align-items: center;
	gap: 20px;
}

.input-name {
	overflow-x: hidden;
	width: 200px;
	/* background-color: red; */
	opacity: 60%;
}
</style>
