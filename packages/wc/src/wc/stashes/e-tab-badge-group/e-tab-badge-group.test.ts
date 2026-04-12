import { test, expect, describe, beforeEach } from "vite-plus/test";

import "../e-tab-badge/e-tab-badge.js";
import "./e-tab-badge-group.js";

describe("<e-tab-badge-group>", () => {
  let el: HTMLElementTagNameMap["e-tab-badge-group"];

  beforeEach(async () => {
    await customElements.whenDefined("e-tab-badge-group");
    document.body.innerHTML = "";
    el = document.createElement("e-tab-badge-group");
    el.stashes = [
      { id: "1", name: "Tab1", type: "PremiumStash", index: 0, metadata: { colour: "ff" } },
      { id: "2", name: "Tab2", type: "PremiumStash", index: 1, metadata: { colour: "00" } },
    ];
    document.body.append(el);
    await el.updateComplete;
  });

  test("should render element with provided stash tabs", () => {
    const badges = el.shadowRoot!.querySelectorAll("e-tab-badge");
    expect(badges.length).to.equal(2);
  });
});
