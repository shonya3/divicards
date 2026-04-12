import { test, expect, vi, describe, beforeEach } from "vitest";
import { page, userEvent } from "vitest/browser";

import { MockStashLoader } from "./data.js";
import "./e-stashes-view.js";

describe("<e-stashes-view>", () => {
  let el: HTMLElementTagNameMap["e-stashes-view"];

  beforeEach(async () => {
    await customElements.whenDefined("e-stashes-view");
    document.body.innerHTML = "";
    el = document.createElement("e-stashes-view");
    el.stashLoader = new MockStashLoader();
    document.body.append(el);

    await el.updateComplete;
  });

  test("should decrement loads available on stash tab loaded", async () => {
    const loadStashBtn = page.getByRole("button", { name: "Load Stash" });
    await expect.element(loadStashBtn).toBeVisible();
    await userEvent.click(loadStashBtn);

    const loadsAvailableInitial = page.getByText("Loads available:30");
    await expect.element(loadsAvailableInitial).toBeVisible();

    const stashTab = page.getByRole("listitem").getByRole("button").first();
    await userEvent.click(stashTab);

    const extractCardsBtn = page.getByRole("button", { name: "Extract cards sample" });
    await expect.element(extractCardsBtn).toBeVisible();
    await userEvent.click(extractCardsBtn);

    const loadsAvailable = page.getByText("Loads available:29");
    await expect.element(loadsAvailable).toBeVisible();
  });

  test("should load stash tabs and emit stashes__extract-cards when Extract cards sample button clicked", async () => {
    const loadStashBtn = page.getByRole("button", { name: "Load Stash" });
    await expect.element(loadStashBtn).toBeVisible();
    await userEvent.click(loadStashBtn);

    const stashTab = page.getByRole("listitem").getByRole("button").first();
    await userEvent.click(stashTab);

    const extractCardsBtn = page.getByRole("button", { name: "Extract cards sample" });
    await expect.element(extractCardsBtn).toBeVisible();

    const spy = vi.fn();
    el.addEventListener("stashes__extract-cards", spy);
    await userEvent.click(extractCardsBtn);

    expect(spy).toHaveBeenCalledOnce();
  });
});
