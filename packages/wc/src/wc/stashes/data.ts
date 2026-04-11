import { DivinationCardsSample, League } from "@divicards/shared/types.js";
import { IStashLoader } from "@divicards/shared/IStashLoader.js";

import stashesData from "./json/stashes.json" with { type: "json" };
import sampleData from "./json/sample.json" with { type: "json" };
import { NoItemsTab, TabWithItems } from "poe-custom-elements/types.js";

export const stashes = stashesData as NoItemsTab[];
export const league: League = "Standard";
export const sample: DivinationCardsSample = sampleData;
import quadStash from "./json/QuadStashStd.json" with { type: "json" };

export class MockStashLoader implements IStashLoader {
  async tab(_tabId: string, _league: string): Promise<TabWithItems> {
    const quad = quadStash as TabWithItems;
    return quad;
  }
  async sampleFromTab(_tabId: string, _league: League): Promise<DivinationCardsSample> {
    return sample;
  }
  tabs(_league: League): Promise<NoItemsTab[]> {
    return new Promise((r) => r(stashes));
  }
}
