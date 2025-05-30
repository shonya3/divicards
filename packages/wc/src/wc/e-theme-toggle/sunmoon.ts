import { css, CSSResult } from 'lit';

export const sunmoon: CSSResult = css`
	/* @import 'https://unpkg.com/open-props/easings.min.css'; */
	:host {
		--ease-1: cubic-bezier(0.25, 0, 0.5, 1);
		--ease-2: cubic-bezier(0.25, 0, 0.4, 1);
		--ease-3: cubic-bezier(0.25, 0, 0.3, 1);
		--ease-4: cubic-bezier(0.25, 0, 0.2, 1);
		--ease-5: cubic-bezier(0.25, 0, 0.1, 1);
		--ease-in-1: cubic-bezier(0.25, 0, 1, 1);
		--ease-in-2: cubic-bezier(0.5, 0, 1, 1);
		--ease-in-3: cubic-bezier(0.7, 0, 1, 1);
		--ease-in-4: cubic-bezier(0.9, 0, 1, 1);
		--ease-in-5: cubic-bezier(1, 0, 1, 1);
		--ease-out-1: cubic-bezier(0, 0, 0.75, 1);
		--ease-out-2: cubic-bezier(0, 0, 0.5, 1);
		--ease-out-3: cubic-bezier(0, 0, 0.3, 1);
		--ease-out-4: cubic-bezier(0, 0, 0.1, 1);
		--ease-out-5: cubic-bezier(0, 0, 0, 1);
		--ease-in-out-1: cubic-bezier(0.1, 0, 0.9, 1);
		--ease-in-out-2: cubic-bezier(0.3, 0, 0.7, 1);
		--ease-in-out-3: cubic-bezier(0.5, 0, 0.5, 1);
		--ease-in-out-4: cubic-bezier(0.7, 0, 0.3, 1);
		--ease-in-out-5: cubic-bezier(0.9, 0, 0.1, 1);
		--ease-elastic-1: cubic-bezier(0.5, 0.75, 0.75, 1.25);
		--ease-elastic-2: cubic-bezier(0.5, 1, 0.75, 1.25);
		--ease-elastic-3: cubic-bezier(0.5, 1.25, 0.75, 1.25);
		--ease-elastic-4: cubic-bezier(0.5, 1.5, 0.75, 1.25);
		--ease-elastic-5: cubic-bezier(0.5, 1.75, 0.75, 1.25);
		--ease-squish-1: cubic-bezier(0.5, -0.1, 0.1, 1.5);
		--ease-squish-2: cubic-bezier(0.5, -0.3, 0.1, 1.5);
		--ease-squish-3: cubic-bezier(0.5, -0.5, 0.1, 1.5);
		--ease-squish-4: cubic-bezier(0.5, -0.7, 0.1, 1.5);
		--ease-squish-5: cubic-bezier(0.5, -0.9, 0.1, 1.5);
		--ease-step-1: steps(2);
		--ease-step-2: steps(3);
		--ease-step-3: steps(4);
		--ease-step-4: steps(7);
		--ease-step-5: steps(10);
	}

	.sun-and-moon > :is(.moon, .sun, .sun-beams) {
		transform-origin: center center;
	}
	.sun-and-moon > :is(.moon, .sun) {
		fill: var(--icon-fill);
	}
	.theme-toggle:is(:hover, :focus-visible) > .sun-and-moon > :is(.moon, .sun) {
		fill: var(--icon-fill-hover);
	}
	.sun-and-moon > .sun-beams {
		stroke: var(--icon-fill);
		stroke-width: 2px;
	}
	.theme-toggle:is(:hover, :focus-visible) .sun-and-moon > .sun-beams {
		stroke: var(--icon-fill-hover);
	}
	:host([theme='dark']) .sun-and-moon > .sun {
		transform: scale(1.75);
	}
	:host([theme='dark']) .sun-and-moon > .sun-beams {
		opacity: 0;
	}
	:host([theme='dark']) .sun-and-moon > .moon > circle {
		transform: translate(-7px);
	}
	@supports (cx: 1) {
		:host([theme='dark']) .sun-and-moon > .moon > circle {
			transform: translate(0);
			cx: 17;
		}
	}
	@media (prefers-reduced-motion: no-preference) {
		.sun-and-moon > .sun {
			transition: transform 0.5s var(--ease-elastic-3);
		}
		.sun-and-moon > .sun-beams {
			transition: transform 0.5s var(--ease-elastic-4), opacity 0.5s var(--ease-3);
		}
		.sun-and-moon .moon > circle {
			transition: transform 0.25s var(--ease-out-5);
		}
		@supports (cx: 1) {
			.sun-and-moon .moon > circle {
				transition: cx 0.25s var(--ease-out-5);
			}
		}
		:host([theme='dark']) .sun-and-moon > .sun {
			transform: scale(1.75);
			transition-timing-function: var(--ease-3);
			transition-duration: 0.25s;
		}
		:host([theme='dark']) .sun-and-moon > .sun-beams {
			transform: rotate(-25deg);
			transition-duration: 0.15s;
		}
		:host([theme='dark']) .sun-and-moon > .moon > circle {
			transition-delay: 0.25s;
			transition-duration: 0.5s;
		}
	}
	.theme-toggle {
		background: none;
		border: none;
		padding: 0;
		inline-size: var(--size);
		block-size: var(--size);
		aspect-ratio: 1;
		border-radius: 50%;
		cursor: pointer;
		touch-action: manipulation;
		-webkit-tap-highlight-color: transparent;
		outline-offset: 5px;
	}
	.theme-toggle > svg {
		inline-size: 100%;
		block-size: 100%;
		stroke-linecap: round;
	}
	:host([theme='dark']) .theme-toggle {
		--icon-fill: hsl(210 10% 70%);
		--icon-fill-hover: hsl(210 15% 90%);
	}
	* {
		box-sizing: border-box;
		margin: 0;
	}
	html {
		block-size: 100%;
		color-scheme: light;
	}
	html[data-theme='dark'] {
		color-scheme: dark;
	}
	@supports not (color-scheme: dark) {
		html[data-theme='dark'] {
			background: #111;
		}
	}

	.github-corner {
		fill: #ff69b4;
		color: Canvas;
	}
	.github-corner:hover .octo-arm {
		-webkit-animation: octocat-wave 0.56s ease-in-out;
		animation: octocat-wave 0.56s ease-in-out;
	}
	:host([theme='dark']) .github-corner {
		fill: indigo;
	}
	@-webkit-keyframes octocat-wave {
		0%,
		to {
			transform: rotate(0);
		}
		20%,
		60% {
			transform: rotate(-25deg);
		}
		40%,
		80% {
			transform: rotate(10deg);
		}
	}
	@keyframes octocat-wave {
		0%,
		to {
			transform: rotate(0);
		}
		20%,
		60% {
			transform: rotate(-25deg);
		}
		40%,
		80% {
			transform: rotate(10deg);
		}
	}
`;
