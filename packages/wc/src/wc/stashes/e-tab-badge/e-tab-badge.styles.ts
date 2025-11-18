import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	.tab-badge-as-button {
		padding: 0.3rem 0.6rem;
		border-radius: 0.35rem;
		border: 2px solid;
		border-color: var(--badge-color);
		cursor: pointer;
		overflow: hidden;
		position: relative;
		&:disabled {
			filter: grayscale(0.6);
		}
		.name {
			pointer-events: none;
		}

		background-color: color-mix(in srgb, var(--badge-color) 40%, transparent);

		&:hover {
			background-color: color-mix(in srgb, var(--badge-color) 65%, transparent);
		}

		&::after {
			display: block;
			position: absolute;
			bottom: 0;
			right: 0;
			color: var(--sl-color-neutral-900);
			content: var(--tab-index);
			text-align: center;
			border-top-left-radius: 2rem;
			font-size: 0.6rem;
			min-width: 1rem;
		}

		& .name {
			font-size: 0.95rem;
			color: var(--sl-color-neutral-950);

			& .remove-only {
				font-size: 50%;
			}
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

		& .name {
			font-size: 0.85rem;
			color: #000;

			& .remove-only {
				font-size: 60%;
			}
		}
	}

	.name {
		color: var(--badge-color);
		position: relative;

		.remove-only {
			vertical-align: sub;
		}
	}
`;
