<script setup lang="ts">
import HelpTip from './HelpTip.vue';
</script>

<template>
	<div style="display: flex; gap: 1rem">
		<div class="drop">Drop files <span>Here!</span></div>
		<HelpTip>
			<p>Excel, .csv or just .txt</p>
			<p>Required headers: name and amount</p>
			<img src="/simple.png" alt="" />
		</HelpTip>
	</div>
</template>

<style scoped>
.drop {
	font-size: 3rem;
	margin-bottom: 1rem;
}

.drop > span {
	color: deeppink;
	font-weight: 700;
}
</style>
