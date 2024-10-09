import type { DefineComponent } from "vue";

type SampleCardElementProps = {
  /**  */
  league?: TradeLeague;
  /**  */
  filename?: string;
  /**  */
  selected?: boolean | null;
  /**  */
  uuid?: string;
  /**  */
  "minimum-card-price"?: number;
  /**  */
  sample?: DivinationCardsSample;
  /**  */
  tablePopup?: BasePopupElement;
  /**  */
  selectedCheckbox?: HTMLInputElement;
  /**  */
  leagueSelect?: LeagueSelectElement;
  /**  */
  priceSlider?: HTMLInputElement;
  /**  */
  table?: SampleTableElement;
  /**  */
  rangeEl?: SlRange;
  /** Export sample form popup. */
  form_popup?: BasePopupElement;
  /**  */
  export_sample_form_state?: object;
  /** Export the cards sample to file or to google sheets */
  export_sample_to?: ExportSampleTo;
  /**  */
  filteredCards?: string;
  /**  */
  filteredSummary?: string;
  /**  */
  onundefined?: (e: CustomEvent<SubmitExportSampleEvent>) => void;
};

export type CustomElements = {
  /**
   *
   * ---
   *
   *
   * ### **Events:**
   *
   */
  "e-sample-card": DefineComponent<SampleCardElementProps>;
};

declare module "vue" {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface GlobalComponents extends CustomElements {}
}

declare global {
  namespace JSX {
    // eslint-disable-next-line @typescript-eslint/no-empty-interface
    interface IntrinsicElements extends CustomElements {}
  }
}
