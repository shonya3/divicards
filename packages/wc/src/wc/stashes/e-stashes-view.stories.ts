import { html, TemplateResult } from "lit";

import { MockStashLoader } from "./data.js";
import "./e-stashes-view.js";

export default {
  title: "Elements/stashes/stashes-view",
};

export const Default = {
  render(): TemplateResult {
    return html`<e-stashes-view .stashLoader=${new MockStashLoader()}></e-stashes-view>`;
  },
};
