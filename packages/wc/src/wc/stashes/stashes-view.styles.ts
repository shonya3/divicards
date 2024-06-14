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
		border: 0.5px solid rgba(0, 0, 0, 0.6);
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
		position: relative;
	}

	.msg {
		position: absolute;
		font-size: 2rem;
		max-width: max-content;
		margin-inline: auto;
		visibility: hidden;
		margin-block: 0;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}

	.visible {
		visibility: visible;
	}

	.not-visible {
		visibility: hidden;
	}

	.hidden {
		display: none;
	}
`;
