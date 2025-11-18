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

		& .header__left {
			display: flex;
			flex-wrap: wrap;
			align-items: center;
			gap: 1rem;
			flex-grow: 1;

			& sl-input {
				width: 15ch;
				margin-top: 18px;
			}
		}

		.header__right {
			display: flex;
			align-items: center;
			gap: 1rem;
		}
	}

	.tabs-total__count {
		color: var(--sl-color-amber-800);
	}

		.list {
			display: flex;
			flex-wrap: wrap;
			list-style: none;
			gap: 0.5rem;
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
