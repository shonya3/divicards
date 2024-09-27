import json from './props.json' assert { type: 'json' };
import { Props } from './e-sample-card';

export const props: Props = json as Props;
export const { league, filename, selected, uuid, minimumCardPrice, sample } = props;
