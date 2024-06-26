import { css } from 'lit';

export const styles = css`
	:host {
		display: block;
		max-width: 1500px;
	}

	.main-stashes-component {
		position: relative;
		padding: 1rem;
		padding: 0.6rem;
		border: 0.5px solid var(--sl-color-gray-200);
		border-radius: 0.25rem;
	}

	wc-help-tip::part(tooltip) {
		right: 5px;
	}

	.header {
		display: flex;
		justify-content: space-between;
	}

	.tips {
		display: flex;
		gap: 0.5rem;
		margin-top: 0.75rem;
	}

	.top-right-corner {
		display: flex;
		gap: 2rem;
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

	poe-stash-tab {
		display: block;
		margin-top: 1.5rem;
		margin-inline: auto;
	}
`;
