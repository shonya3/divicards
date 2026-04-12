import { test, expect, vi, describe, beforeEach } from "vitest";
import { page, userEvent } from "vitest/browser";

import { cards } from "./data.js";
import "./e-sample-table.js";

describe("<e-sample-table>", () => {
  let el: HTMLElementTagNameMap["e-sample-table"];

  beforeEach(async () => {
    await customElements.whenDefined("e-sample-table");
    document.body.innerHTML = "";
    el = document.createElement("e-sample-table");
    el.cards = cards;
    document.body.append(el);
    await el.updateComplete;
  });

  test("should emit sample-table__change:min_price on slider change", async () => {
    const spy = vi.fn();
    const minPrice = 100;

    el.addEventListener("sample-table__change:min_price", (e) => {
      expect(e.$min_price).toBe(minPrice);
      spy();
    });

    const slider = page.getByRole("slider", { name: "min price" });
    await userEvent.fill(slider, String(minPrice));

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should filter cards by name query", async () => {
    const textbox = page.getByRole("textbox", { name: "enter name" });
    await userEvent.fill(textbox, "The");
    await userEvent.tab();

    const table = page.getByRole("table");
    await expect.element(table).toBeVisible();
  });

  test("should emit sample-table__change:column-order on triangle click", async () => {
    const spy = vi.fn();

    el.addEventListener("sample-table__change:column-order", (e) => {
      expect(e.$column).toBeDefined();
      expect(e.$order).toBeDefined();
      spy();
    });

    const table = page.getByRole("table");
    await expect.element(table).toBeVisible();

    const triangle = table.getByRole("button").first();
    await expect.element(triangle).toBeVisible();

    await userEvent.click(triangle);

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should render table with cards", async () => {
    const table = page.getByRole("table");
    await expect.element(table).toBeVisible();
  });
});
