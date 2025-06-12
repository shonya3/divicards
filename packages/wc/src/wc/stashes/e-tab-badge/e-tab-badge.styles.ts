import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	.tab-badge-as-button {
		background-color: var(--badge-color);
		max-width: 5.5rem;
		min-width: 3rem;
		height: 2.2rem;
		border-radius: 0.4rem;
		border: 1px solid #000;
		cursor: pointer;
		overflow: hidden;
		position: relative;
		&:hover {
		}
		&:disabled {
			filter: grayscale(0.6);
		}
		.name {
			pointer-events: none;
		}

		&::after {
			display: block;
			position: absolute;
			bottom: 0;
			right: 0;
			background-color: rgba(255, 255, 255, 0.06);
			color: #000;
			content: var(--tab-index);
			text-align: center;
			border-top-left-radius: 2rem;
			font-size: 0.6rem;
			min-width: 1rem;
		}
	}

	.name {
		color: var(--badge-color);
		font-size: 0.85rem;
		color: #000;
		position: relative;

		.remove-only {
			font-size: 60%;
			vertical-align: sub;
		}
	}

	.tab-badge-as-checkbox {
		width: 5.5rem;
		height: 2.2rem;
		aspect-ratio: 1;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 0.4rem;
		border: 1px solid #000;
		overflow: clip;
		background-color: var(--badge-color);
		position: relative;
		&:has(.checkbox:checked) {
			transform: scale(1.3);
			z-index: 2;
		}

		.checkbox {
			position: absolute;
			appearance: none;
			height: 100%;
			width: 100%;
			cursor: pointer;
		}

		&::after {
			display: block;
			position: absolute;
			bottom: 0;
			right: 0;
			background-color: rgba(255, 255, 255, 0.06);
			color: #000;
			content: var(--tab-index);
			text-align: center;
			border-top-left-radius: 2rem;
			font-size: 0.6rem;
			min-width: 1rem;
		}
	}
`;
