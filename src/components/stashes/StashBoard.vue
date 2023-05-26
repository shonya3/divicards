<script setup lang="ts"></script>

<template>
	<div class="stash-board"></div>
</template>

<style scoped>
.stash-board {
	display: grid;
	grid-template-columns: repeat(12, 1fr);
	grid-template-rows: repeat(12, 1fr);
	width: 570px;
	height: 570px;
	background-color: aqua;
}
</style>
