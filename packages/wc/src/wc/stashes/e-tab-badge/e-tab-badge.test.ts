import { test, expect, vi, describe, beforeEach } from "vitest";
import { page, userEvent } from "vitest/browser";

import "./e-tab-badge.js";
import { TabClickEvent, TabSelectEvent } from "./events.js";

describe("<e-tab-badge>", () => {
  let el: HTMLElementTagNameMap["e-tab-badge"];

  beforeEach(async () => {
    await customElements.whenDefined("e-tab-badge");
    document.body.innerHTML = "";
    el = document.createElement("e-tab-badge");
    el.tab = {
      id: "Test id",
      name: "Heist",
      type: "PremiumStash",
      index: 0,
      metadata: { colour: "ff" },
    };
    document.body.append(el);
    await el.updateComplete;
  });

  test("should emit stashes__tab-click on element click when as=button", async () => {
    const spy = vi.fn();

    el.addEventListener("stashes__tab-click", (e: Event) => {
      expect(e instanceof TabClickEvent).toBeTruthy();
      spy();
    });

    await userEvent.click(el);

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should NOT emit stashes__tab-click on element click when as=checkbox", async () => {
    el.as = "checkbox";
    await el.updateComplete;

    const clickHandler = vi.fn();
    el.addEventListener("stashes__tab-click", clickHandler);

    await userEvent.click(el);

    expect(clickHandler).not.toHaveBeenCalled();
  });

  test("should emit stashes__tab-select on checkbox change", async () => {
    el.as = "checkbox";
    await el.updateComplete;

    const spy = vi.fn();
    el.addEventListener("stashes__tab-select", (e) => {
      expect(e instanceof TabSelectEvent).toBeTruthy();
      spy();
    });

    const checkbox = page.getByRole("checkbox");
    await userEvent.click(checkbox);

    expect(spy).toHaveBeenCalledOnce();
  });
});
