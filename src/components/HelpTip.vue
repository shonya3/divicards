<script setup lang="ts"></script>

<template>
	<div class="help-tip">
		<div class="tooltip">
			<slot>
				<p>Excel, .csv or just .txt</p>
				<p>Requirement: "name" and "amount" headers</p>
				<img src="/simple.png" alt="tip" />
			</slot>
		</div>
	</div>
</template>

<style scoped>
.help-tip {
	position: relative;
	text-align: center;
	background-color: deeppink;
	border-radius: 50%;
	width: 24px;
	height: 24px;
	line-height: 26px;
	cursor: default;
}

.help-tip:before {
	content: '?';
	font-weight: bold;
	color: #fff;
}

.help-tip:hover .tooltip {
	display: block;
	transform-origin: 100% 0%;

	-webkit-animation: fadeIn 0.3s ease-in-out;
	animation: fadeIn 0.3s ease-in-out;
}

.tooltip {
	/* The tooltip */
	display: none;
	text-align: left;
	background-color: #1e2021;
	padding: 20px;
	min-width: 300px;
	position: absolute;
	border-radius: 3px;
	box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.2);
	right: -4px;
	color: #fff;
	font-size: 1rem;
	line-height: 1.4;
	z-index: 2;
}

.help-tip p:before {
	/* The pointer of the tooltip */
	position: absolute;
	content: '';
	width: 0;
	height: 0;
	border: 6px solid transparent;
	border-bottom-color: #1e2021;
	right: 10px;
	top: -12px;
}

.help-tip p:after {
	/* Prevents the tooltip from being hidden */
	width: 100%;
	height: 40px;
	content: '';
	position: absolute;
	top: -40px;
	left: 0;
}

/* CSS animation */

@-webkit-keyframes fadeIn {
	0% {
		opacity: 0;
		transform: scale(0.6);
	}

	100% {
		opacity: 100%;
		transform: scale(1);
	}
}

@keyframes fadeIn {
	0% {
		opacity: 0;
	}
	100% {
		opacity: 100%;
	}
}
</style>
