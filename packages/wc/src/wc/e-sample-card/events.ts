import { DivinationCardsSample, League } from '@divicards/shared/types.js';
import { PresubmitExportFormEvent, ExportFormArgs } from './e-form-export-sample/e-form-export-sample.js';
import { LeagueChangeEvent } from '../events/change/league.js';

export type Events = [
	typeof SubmitExportSampleEvent,
	typeof SelectedChangeEvent,
	typeof MinimumCardsPriceChangeEvent,
	typeof DeleteThisSampleEvent,
	typeof LeagueChangeEvent,
	typeof SaveToFileClickEvent,
	typeof GoogleSheetsClickEvent
];

declare global {
	interface HTMLElementEventMap {
		sample__delete: DeleteThisSampleEvent;
	}
}
export class DeleteThisSampleEvent extends Event {
	static readonly tag = 'sample__delete';
	readonly $uuid: string;
	constructor(uuid: string, options?: EventInit) {
		super(DeleteThisSampleEvent.tag, options);
		this.$uuid = uuid;
	}
}

declare global {
	interface HTMLElementEventMap {
		'sample__change:selected': SelectedChangeEvent;
	}
}
export class SelectedChangeEvent extends Event {
	static readonly tag = 'sample__change:selected';
	readonly $selected: boolean | null;
	constructor(selected: boolean | null, options?: EventInit) {
		super(SelectedChangeEvent.tag, options);
		this.$selected = selected;
	}
}

declare global {
	interface HTMLElementEventMap {
		'sample__change:minimum_card_price': MinimumCardsPriceChangeEvent;
	}
}
export class MinimumCardsPriceChangeEvent extends Event {
	static readonly tag = 'sample__change:minimum_card_price';
	readonly $minimum_card_price: number;

	constructor(minimum_card_price: number, options?: EventInit) {
		super(MinimumCardsPriceChangeEvent.tag, options);
		this.$minimum_card_price = minimum_card_price;
	}
}

declare global {
	interface HTMLElementEventMap {
		'sample__google-sheets-click': GoogleSheetsClickEvent;
	}
}
export class GoogleSheetsClickEvent extends Event {
	static readonly tag = 'sample__google-sheets-click';
	readonly $sample: DivinationCardsSample;
	readonly $league: League;

	constructor(sample: DivinationCardsSample, league: League, options?: EventInit) {
		super(GoogleSheetsClickEvent.tag, options);
		this.$sample = sample;
		this.$league = league;
	}
}

declare global {
	interface HTMLElementEventMap {
		'sample__save-to-file-click': SaveToFileClickEvent;
	}
}
export class SaveToFileClickEvent extends Event {
	static readonly tag = 'sample__save-to-file-click';
	readonly $sample: DivinationCardsSample;
	readonly $league: League;
	readonly $filename: string;

	constructor(
		args: {
			sample: DivinationCardsSample;
			league: League;
			filename: string;
		},
		options?: EventInit
	) {
		super(SaveToFileClickEvent.tag, options);
		this.$sample = args.sample;
		this.$league = args.league;
		this.$filename = args.filename;
	}
}

declare global {
	interface HTMLElementEventMap {
		'sample__submit-export-sample': SubmitExportSampleEvent;
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
