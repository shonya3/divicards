export interface DivTableProps {
	cards: DivinationCardRecord[];
}

import { DivinationCardRecord } from '@divicards/shared/types';
import props from './div-table.props.json' assert { type: 'json' };

export const divTableProps: DivTableProps = props;
export const cards = props.cards;