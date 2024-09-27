import { css } from 'lit';

const table = css`
	.table-wrapper {
		grid-area: table;
		height: 80vh;
		overflow: auto;
		width: fit-content;
	}
    
    tbody {
		overflow-y: scroll;
	}

	table {
		border-collapse: collapse;
        background-color: background-color: var(--sl-color-gray-50);
	}

 
    .th-number, .td-number {
        width: 40px;
    }
    .th-amount, .td-amount {
        width: 70px;
    }
    .th-name, .td-name {
        width: 230px;
    }
    .th-price{}
    .th-sum{}
    .th-weight, .td-weight{
    }

    thead, tbody tr {
        display: table;
        width: 730px;
        table-layout: fixed;/* even columns width , fix width of table too*/
    }

	thead {
		position: sticky;
		top: -1px;
		background-color: var(--sl-color-gray-50);
		z-index: 10;
		scroll-padding-top: 1000px;
		border: 1px solid var(--sl-color-gray-300);
	}

    .th,
	.td {
		padding: 1rem;
		border: 1px solid var(--sl-color-gray-300);
		text-align: center;
	}

	.th {
		font-weight: 500;
		color: var(--sl-color-gray-800);

		& > div {
			display: flex;
			align-items: center;
			gap: 0.5rem;
			margin-inline: auto;
			width: fit-content;
		}
	}

	.td {
		color: var(--sl-color-gray-700);
		--poe-item-size: 1rem;
		& > div {
			display: flex;
			align-items: center;
			gap: 0.5rem;
		}
	}

	tr {
		&:hover {
			outline: 1px var(--sl-color-blue-500) solid;
		}

		&:hover .td {
			background-color: var(--sl-color-blue-100);
		}
	}
`;

export const styles = css`
	:host {
		display: block;
		max-width: 1220px;
		background-color: var(--bg-color, #242424);
	}

	.ch-3 {
		text-align: center;
		min-width: 3ch;
	}

	.layout {
		height: 100%;
		max-width: 1220px;
		background-color: var(--bg-color, #242424);
		color: var(--color, rgba(255, 255, 255, 0.87));

		padding: 2rem;
		display: grid;
		grid-template:
			'header not-header'
			'table stats' auto / 750px 1fr;
	}

	.header {
		grid-area: header;
		position: sticky;
		top: 0;
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		z-index: 2;
		background-color: var(--bg-color, #242424);
		padding-top: 20px;
		padding-bottom: 20px;
		border-bottom: 1px solid var(--sl-color-gray-100);
	}

	.slider-box {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
	}

	.stats {
		grid-area: stats;
		display: flex;
		flex-direction: column;
		align-items: center;
		list-style: none;
		gap: 2rem;
		padding: 1rem;
	}

	.stat {
		padding: 2rem;
		display: flex;
		align-items: center;
		width: 200px;
		border: 1px solid var(--sl-color-gray-300);
		border-radius: 0.25rem;
		font-size: 2rem;
		font-weight: 500;
		gap: 1.5rem;
	}

	${table}
`;
