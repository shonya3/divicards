import { html, TemplateResult } from "lit";

import { Meta } from "@storybook/web-components-vite";

import { DropFilesMessageElement } from "./e-drop-files-message.js";
import "./e-drop-files-message.js";

const meta: Meta<DropFilesMessageElement> = {
  title: "Elements/drop-files-message",
};
export default meta;

export const Default = {
  render(): TemplateResult {
    return html`<e-drop-files-message></e-drop-files-message>`;
  },
};
