import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	.theme-toggle {
		--size: 2rem;
		--icon-fill: hsl(210 10% 35%);
		--icon-fill-hover: hsl(210 10 15%);

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

	.sun-and-moon > :is(.moon, .sun, .sun-beams) {
		transform-origin: center center;
	}

	.sun-and-moon > :is(.moon, .sun) {
		fill: var(--icon-fill);
	}
`;
