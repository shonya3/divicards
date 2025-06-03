import { NoItemsTab } from 'poe-custom-elements/types.js';

export type ErrorLabel = { noItemsTab: NoItemsTab; message: string };

export type SelectedStashtabs = Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;
