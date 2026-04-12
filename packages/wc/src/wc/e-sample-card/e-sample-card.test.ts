import { test, expect, vi, describe, beforeEach } from "vite-plus/test";
import { page, userEvent } from "vite-plus/test/browser";

import { props } from "./data.js";
import "./e-sample-card.js";

describe("<e-sample-card>", () => {
  let el: HTMLElementTagNameMap["e-sample-card"];

  beforeEach(async () => {
    await customElements.whenDefined("e-sample-card");
    document.body.innerHTML = "";
    el = document.createElement("e-sample-card");
    Object.assign(el, props);
    document.body.append(el);
    await el.updateComplete;
  });

  test("should emit sample__change:selected on checkbox click", async () => {
    const checkbox = page.getByRole("checkbox");
    await expect.element(checkbox).toBeVisible();

    const spy = vi.fn();
    el.addEventListener("sample__change:selected", (e) => {
      expect(e.$selected).toBeTruthy();
      spy();
    });

    await userEvent.click(checkbox, { force: true });

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should emit sample__delete on clicking delete sample button", async () => {
    const btnDelete = page.getByRole("button", { name: "delete sample" });
    await expect.element(btnDelete).toBeVisible();

    const spy = vi.fn();
    el.addEventListener("sample__delete", spy);

    await userEvent.click(btnDelete);

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should emit sample__google-sheets-click on button click", async () => {
    const btnGoogleSheets = page.getByRole("button", { name: "Export to Google Sheets" });
    await expect.element(btnGoogleSheets).toBeVisible();

    const spy = vi.fn();
    el.addEventListener("sample__google-sheets-click", spy);

    await userEvent.click(btnGoogleSheets);

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should emit sample__save-to-file-click on button click", async () => {
    const btnSaveToFile = page.getByRole("button", { name: "save to file" });
    await expect.element(btnSaveToFile).toBeVisible();

    const spy = vi.fn();
    el.addEventListener("sample__save-to-file-click", spy);

    await userEvent.click(btnSaveToFile);

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should emit sample__change:minimum_card_price on slider change", async () => {
    const spy = vi.fn();
    const minPrice = 100;

    el.addEventListener("sample__change:minimum_card_price", (e) => {
      expect(e.$minimum_card_price).toBe(minPrice);
      spy();
    });

    const sliderMinPrice = page.getByRole("slider", { name: "min card price in chaos" });
    await userEvent.fill(sliderMinPrice, String(minPrice));

    expect(spy).toHaveBeenCalledOnce();
  });

  test("should emit sample__change:filename on input change", async () => {
    const spy = vi.fn();
    const newName = "my-sample";
    el.addEventListener("sample__change:filename", (e) => {
      expect(e.$filename).toBe(newName);
      spy();
    });

    const textbox = page.getByRole("textbox", { name: "Edit filename" });

    await userEvent.fill(textbox, newName);
    await userEvent.tab();

    expect(spy).toHaveBeenCalled();
  });

  test("should emit sample__submit-export-sample on export form submit", async () => {
    const spy = vi.fn();
    el.addEventListener("sample__submit-export-sample", spy);

    const btnSaveToFile = page.getByRole("button", { name: "Save to file" });
    await expect.element(btnSaveToFile).toBeVisible();
    await userEvent.click(btnSaveToFile);

    const btnSubmit = page.getByRole("button", { name: "Submit" });
    await expect.element(btnSubmit).toBeVisible();
    await userEvent.click(btnSubmit);

    expect(spy).toHaveBeenCalledOnce();
  });
});
