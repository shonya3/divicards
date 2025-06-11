import { DivinationCardsSample, League } from '@divicards/shared/types.js';
import { PresubmitExportFormEvent, ExportFormArgs } from './e-form-export-sample/e-form-export-sample.js';
import { LeagueChangeEvent } from '../events/change/league.js';
import { EventMapFrom } from '../../event-utils.js';

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export type Events = [
	typeof SubmitExportSampleEvent,
	typeof SelectedChangeEvent,
	typeof MinimumCardsPriceChangeEvent,
	typeof DeleteThisSampleEvent,
	typeof LeagueChangeEvent,
	typeof SaveToFileClickEvent,
	typeof GoogleSheetsClickEvent,
	typeof FilenameChangeEvent
];

export class DeleteThisSampleEvent extends Event {
	static readonly tag = 'sample__delete';

	constructor(readonly $uuid: string, options?: EventInit) {
		super(DeleteThisSampleEvent.tag, options);
	}
}

export class SelectedChangeEvent extends Event {
	static readonly tag = 'sample__change:selected';

	constructor(readonly $selected: boolean | null, options?: EventInit) {
		super(SelectedChangeEvent.tag, options);
	}
}

export class MinimumCardsPriceChangeEvent extends Event {
	static readonly tag = 'sample__change:minimum_card_price';

	constructor(readonly $minimum_card_price: number, options?: EventInit) {
		super(MinimumCardsPriceChangeEvent.tag, options);
	}
}

export class GoogleSheetsClickEvent extends Event {
	static readonly tag = 'sample__google-sheets-click';

	constructor(readonly $sample: DivinationCardsSample, readonly $league: League, options?: EventInit) {
		super(GoogleSheetsClickEvent.tag, options);
	}
}

export class SaveToFileClickEvent extends Event {
	static readonly tag = 'sample__save-to-file-click';

	constructor(
		readonly $sample: DivinationCardsSample,
		readonly $league: League,
		readonly $filename: string,
		options?: EventInit
	) {
		super(SaveToFileClickEvent.tag, options);
	}
}

export class SubmitExportSampleEvent extends PresubmitExportFormEvent {
	static readonly tag = 'sample__submit-export-sample';
	readonly $sample: DivinationCardsSample;
	readonly $league: League;
	readonly $filename: string;

	constructor(
		{
			form_args,
			sample,
			league,
			filename,
		}: { filename: string; form_args: ExportFormArgs; sample: DivinationCardsSample; league: League },
		options?: EventInit
	) {
		super(form_args, SubmitExportSampleEvent.tag, options);
		this.$sample = sample;
		this.$league = league;
		this.$filename = filename;
	}
}

export class FilenameChangeEvent extends Event {
	static readonly tag = 'sample__change:filename';

	constructor(public readonly $filename: string) {
		super(FilenameChangeEvent.tag);
	}
}
