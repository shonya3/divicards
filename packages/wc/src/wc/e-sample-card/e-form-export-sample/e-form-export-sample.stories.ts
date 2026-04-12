import { html, TemplateResult } from "lit";

import { Meta } from "@storybook/web-components-vite";

import { FormExportSampleElement } from "./e-form-export-sample.js";
import "./e-form-export-sample.js";

const meta: Meta<FormExportSampleElement> = {
  title: "Elements/e-sample-card/e-form-export-sample",
};
export default meta;

export const Default = {
  render(): TemplateResult {
    return html`<e-form-export-sample></e-form-export-sample>`;
  },
};
