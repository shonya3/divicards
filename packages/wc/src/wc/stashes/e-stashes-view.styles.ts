import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	:host {
		display: block;
		max-width: 1500px;
		background-color: var(--sl-color-neutral-0);
		box-shadow: 0 2px 8px color-mix(in srgb, var(--sl-color-neutral-1000, black) 6%, transparent),
			0 4px 12px color-mix(in srgb, var(--sl-color-neutral-1000, black) 8%, transparent);
	}

	.main-stashes-component {
		position: relative;
		border-radius: 0.25rem;
		padding-inline: 1rem;
	}

	wc-help-tip::part(tooltip) {
		right: 5px;
	}

	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;

		& e-league-select {
			margin-top: 1rem;
		}

		& .loads-available {
			color: var(--sl-color-neutral-600);
		}

		& .loads-available__value {
			color: var(--sl-color-neutral-700);
		}
	}

	.tips {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.top-right-corner {
		display: flex;
		gap: 1rem;
		align-items: center;
	}

	.btn-load-items:not([disabled]) {
		transform: scale(1.2);
	}

	.messages {
		min-height: 2rem;
	}

	.msg {
		max-width: max-content;
		margin-inline: auto;
		margin-block: 0;
		font-size: 20px;
	}



	e-stash-tab-container {
		display: block;
		margin-inline: auto;
	}
`;
