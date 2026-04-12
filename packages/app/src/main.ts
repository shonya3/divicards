import { setBasePath } from "@shoelace-style/shoelace";
import SlAlrt from "@shoelace-style/shoelace/dist/components/alert/alert.component.js";
import SlButton from "@shoelace-style/shoelace/dist/components/button/button.component.js";
import SlIcon from "@shoelace-style/shoelace/dist/components/icon/icon.component.js";

import "@divicards/wc/e-base-popup.js";
import "@divicards/wc/e-google-auth.js";
import { createPinia } from "pinia";
import { createApp } from "vue";

import App from "./App.vue";
import { handleError } from "./error";
import { addRustListener } from "./event";
import "./style.css";
import { toast } from "./toast";

setBasePath("/");
SlAlrt.define("sl-alert");
SlButton.define("sl-button");
SlIcon.define("sl-icon");

const pinia = createPinia();

const app = createApp(App);
app.use(pinia);
app.mount("#app");

window.addEventListener("unhandledrejection", (event) => handleError(event.reason));
app.config.errorHandler = handleError;
addRustListener("toast", (e) => {
  toast(e.payload.variant, e.payload.message);
});
