import { DivinationCardsSample } from "@divicards/shared/types.js";

import { Props, SampleCardElement } from "./e-sample-card.js";
import json from "./props.json" with { type: "json" };

export const props: Props = json as Props;

export const league: "Mirage" | "Standard" | "Hardcore Mirage" | "Hardcore" | undefined = props.league;
export const filename: string = props.filename;
export const selected: SampleCardElement["selected"] = props.selected;
export const uuid: string = props.uuid;
export const minimumCardPrice: number = props.minimumCardPrice;
export const sample: DivinationCardsSample = props.sample;
export const csvDataForDrag: string = props.csvDataForDrag;
