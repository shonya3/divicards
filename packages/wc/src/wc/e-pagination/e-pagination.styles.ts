import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	.page-controls {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
	}

	.buttons {
		display: flex;
		gap: 0.4rem;
		align-items: center;
	}

	sl-icon-button {
		font-size: 1rem;
	}

	sl-input::part(form-control) {
		display: flex;
		flex-direction: column;
	}

	sl-input::part(form-control-label) {
		order: 2;
		color: var(--sl-color-neutral-500);
		font-size: 0.75rem;
	}

	.per_page-input,
	.page-input {
		margin-top: 1.1rem;
		width: 8ch;
	}

	.current-items-label {
		color: var(--sl-color-gray-600);
		font-size: 0.85rem;
		margin-left: 0.4rem;
	}
`;
