import type { DivinationCardsSample, NameAmount } from "@divicards/shared/types.js";
import type { TabWithItems } from "poe-custom-elements";

export type { DivinationCardsSample, NameAmount } from "@divicards/shared/types.js";

declare module "@divicards/divi-wasm" {
  export function create_sample_from_csv(csv: string): DivinationCardsSample;
  export function create_sample_from_name_amount(pairs: NameAmount[]): DivinationCardsSample;
  export function create_sample_from_tab(tab: TabWithItems): DivinationCardsSample;
  export default function init(): Promise<void>;
}
