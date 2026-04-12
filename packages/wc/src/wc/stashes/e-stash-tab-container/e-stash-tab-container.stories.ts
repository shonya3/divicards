import { html, TemplateResult } from "lit";

import { Meta } from "@storybook/web-components-vite";
import { TabWithItems } from "poe-custom-elements/types.js";

//@ts-ignore
import fragmentsTab from "../json/fragmentsTab.json";
//@ts-ignore
import quadStash from "../json/QuadStashStd.json";
import "./e-stash-tab-container.js";
import { StashTabContainerElement } from "./e-stash-tab-container.js";

const meta: Meta<StashTabContainerElement> = {
  title: "Elements/stashes/e-stash-tab-container",
};
export default meta;

export const Default = {
  render(): TemplateResult {
    return html`<e-stash-tab-container status="complete" .tab=${quadStash as TabWithItems}></e-stash-tab-container>`;
  },
};

export const Fragments = {
  render(): TemplateResult {
    return html`<e-stash-tab-container status="complete" .tab=${fragmentsTab as TabWithItems}></e-stash-tab-container>`;
  },
};
