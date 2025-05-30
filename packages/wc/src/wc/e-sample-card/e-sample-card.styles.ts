import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	:host {
		--border-color: rgba(255, 255, 255, 0.3);
		--border-radius: 0.25rem;
	}

	.sample-card {
		position: relative;
		padding-inline: 1rem;
		padding-top: 1.4rem;
		padding-bottom: 0.4rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		width: fit-content;
		box-shadow: rgba(0, 0, 0, 0.02) 0px 1px 3px 0px, rgba(27, 31, 35, 0.15) 0px 0px 0px 1px;

		/* max-height: 320px; */
		width: 250px;
		height: 530px;

		border: 1px solid black;
		border-color: var(--border-color);
		border-radius: var(--border-radius);
		background-color: var(--sl-color-gray-100);
		transition: 0.2s border-color;
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
		position: absolute;
		top: 0;
		right: 0;
		padding: 0.2rem;
		border: none;
		background-color: transparent;
		cursor: pointer;
	}

	sl-icon {
		color: var(--sl-color-green-600);
	}

	.export-buttons {
		margin-top: 2rem;
		display: flex;
		flex-direction: column;
	}

	.checkbox {
		background-color: red;
		transform: scale(2);
		accent-color: var(--sl-color-green-600);
		cursor: pointer;

		position: absolute;
		bottom: 0;
		right: 0;
		width: 10px;
		height: 10px;
		transform: translate(50%, 50%) scale(2);
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
`;
