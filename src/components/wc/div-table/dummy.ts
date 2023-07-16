export interface DivTableProps {
	cards: DivinationCardRecord[];
}

import { DivinationCardRecord } from '../../../types';
import props from './DivTableProps.json' assert { type: 'json' };

export const divTableProps: DivTableProps = props;
