import fileCardPropsJson from './fileCardProps.json' assert { type: 'json' };
import { FileCardProps } from './file-card';

export const fileCardProps = fileCardPropsJson as FileCardProps;
export const { league, filename, selected, uuid, minimumCardPrice, sample } = fileCardProps;
