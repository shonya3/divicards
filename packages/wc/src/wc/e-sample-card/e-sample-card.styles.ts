import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	:host {
		--border-radius: 0.25rem;
	}

	.sample-card {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		width: fit-content;
		box-shadow: 0 2px 8px color-mix(in srgb, var(--sl-color-neutral-1000, black) 6%, transparent),
			0 4px 12px color-mix(in srgb, var(--sl-color-neutral-1000, black) 8%, transparent);

		width: 250px;
		height: 580px;

		border-radius: var(--border-radius);
		background-color: var(--sl-color-neutral-0);
		padding: 2rem;
		padding-top: 0.5rem;
		transition: 0.2s border-color;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		margin-bottom: 1rem;
	}

	.sample-card--selected {
		border-color: var(--sl-color-green-600);
	}

	.icon {
		cursor: pointer;
	}

	p {
		margin: 0;
	}

	.minor-icons {
		position: absolute;
		top: 30%;
		left: 20px;
		display: flex;
		flex-direction: column;
	}

	.file-error {
		border-color: red;
	}

	.filename {
		font-size: 1rem;
		letter-spacing: -0.4px;
		overflow: hidden;
		max-height: 60px;
		max-width: 100%;
		margin-top: 1.2rem;
	}

	.filename:hover {
		overflow: visible;
	}

	.slider-box {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
	}

	.btn-delete {
		border: none;
		background-color: transparent;
		cursor: pointer;
	}

	.export-buttons {
		margin-top: 2rem;
		display: flex;
		flex-direction: column;
		gap: 0.2rem;

		& sl-icon {
			color: var(--sl-color-green-600);
			font-size: 1.25rem !important;
		}
	}

	#selected-checkbox {
		position: absolute;
		bottom: 0;
		right: 0;
		width: 10px;
		height: 10px;

		&::part(control--checked),
		&::part(control--indeterminate) {
			background-color: var(--sl-color-green-600);
			border-color: var(--sl-color-green-600);
		}

		&::part(checked-icon),
		&::part(indeterminate-icon) {
			color: white;
		}
	}

	.export-to-google-docs {
		margin-top: auto;
		cursor: pointer;
	}

	.total-price,
	.cards-amount {
		display: flex;
		justify-content: center;
		align-items: center;
		font-size: 2rem;
	}

	.grid-icon {
		display: block;
		cursor: pointer;
		padding: 0;
		margin: 0;
		width: 100px;
		height: 100px;
	}

	.drag-handle-container {
		cursor: grab;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem;
		border-radius: var(--sl-border-radius-medium);
	}

	.drag-handle-icon {
		font-size: 1.5rem;
	}
	.drag-handle-text {
		font-size: var(--sl-font-size-small);
	}
`;
