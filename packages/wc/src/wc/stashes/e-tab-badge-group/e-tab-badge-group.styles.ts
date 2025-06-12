import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	:host {
		display: inline-block;
	}
	.tab-badge-group {
		display: grid;
		gap: 1rem;
	}

	.header {
		padding-inline: 2rem;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 2rem;

		.header__right {
			margin-top: 12px;
			display: flex;
			align-items: center;
			gap: 1rem;
		}

		> sl-input {
			margin-top: 1rem;
		}
	}

	.tabs-total__count {
		color: var(--sl-color-amber-800);
	}

	.list {
		display: flex;
		flex-wrap: wrap;
		list-style: none;
		gap: 5px;
		margin-inline: 1rem;
	}

	li {
		border: 1px solid transparent;
		border-radius: 4px;
	}

	.hovered-error {
		border-color: red;
	}
`;
