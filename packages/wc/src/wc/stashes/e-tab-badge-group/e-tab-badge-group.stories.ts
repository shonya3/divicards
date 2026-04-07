import { html, TemplateResult } from "lit";
import "./e-tab-badge-group.js";
import { league, stashes } from "../data.js";

export default {
  title: "Elements/stashes/e-tab-badge-group",
};

export const Default = {
  render(): TemplateResult {
    return html`<e-tab-badge-group .stashes=${stashes} .league=${league}></e-tab-badge-group>`;
  },
};
