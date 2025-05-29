import json from './props.json' with { type: 'json' };
import { Props } from './e-sample-card.js';

export const props: Props = json as Props;
export const { league, filename, selected, uuid, minimumCardPrice, sample } = props;
