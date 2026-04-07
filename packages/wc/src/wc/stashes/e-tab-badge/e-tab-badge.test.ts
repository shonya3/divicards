import { test, expect, vi, describe, beforeEach } from "vitest";
import "./e-tab-badge.js";
import type { TabBadgeElement } from "./e-tab-badge.js";
import { TabClickEvent, TabSelectEvent } from "./events.js";

describe("<e-tab-badge>", () => {
  let el: TabBadgeElement;

  beforeEach(async () => {
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

    return () => {
      el.remove();
    };
  });

  test("should render a component", () => {
    expect(document.querySelector("e-tab-badge")).to.not.be.null;
  });

  test("should emit stashes__tab-click on element click when as=button", async () => {
    const spy = vi.fn();

    el.addEventListener("stashes__tab-click", (e: Event) => {
      expect(e instanceof TabClickEvent).toBeTruthy();
      spy();
    });

    el.click();

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should NOT emit stashes__tab-click on element click when as=checkbox", async () => {
    el.as = "checkbox";
    await el.updateComplete;

    const clickHandler = vi.fn();
    el.addEventListener("stashes__tab-click", clickHandler);

    el.click();

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

    el.checkbox.click();

    expect(spy).toHaveBeenCalledOnce();
  });
});
