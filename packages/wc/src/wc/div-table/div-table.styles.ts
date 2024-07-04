import { css } from 'lit';

export const styles = css`
		:host {
			display: block;
			max-width: 1220px;
			min-height: 100vh;
		}

		.slider-box {
			display: flex;
			justify-content: center;
			align-items: center;
			gap: 0.5rem;
		}
	
		.div-table {
			height: 100%;
			max-width: 1220px;

			color: var(--color, rgba(255, 255, 255, 0.87));
			background-color: var(--bg-color, #242424);
			padding: 1rem;
            display: grid;
            grid-template: 
                "header header" 
                "table stats" auto / 750px 1fr;
		}


		.header {
            grid-area: header;
			position: sticky;
			top: 0;
			display: flex;
			gap: 1rem;
			align-items: center;
			flex-wrap: wrap;
			z-index: 2;
			background-color: var(--bg-color, #242424);
			padding-top: 20px;
			padding-bottom: 20px;
			border-bottom: 1px solid var(--sl-color-gray-100);
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

        .stat{
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

		.table-wrapper {
            grid-area: table;
			height: 80vh;
			overflow: auto;
			width: fit-content;
		}

		tbody {
			max-width: 700px;
			min-width: 610px;
			height: 200px !important;
			overflow-y: scroll;
		}

		.column-name {
			overflow-x: hidden;
			white-space: nowrap;
		}

		table {
			border-collapse: collapse;
			min-width: 650px;
            background-color: background-color: var(--sl-color-gray-50);
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
                background-color:  var(--sl-color-blue-100);

            }
		}

		.ch-3 {
			/* display: block; */
			text-align: center;
			min-width: 3ch;
		}
		.ch-6 {
			text-align: center;
			min-width: 6ch;
		}
		.ch-7 {
			text-align: center;
			min-width: 7ch;
		}
	`;
