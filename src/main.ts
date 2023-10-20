import { createApp } from "vue";
import { createPinia } from "pinia";

import "./main.less";

import Button from "./core/Button.vue";
import Configuration from "./core/Configuration.vue";
import Input from "./core/Input.vue";

import App from "./App.vue";

import router from "./router";

const pinia = createPinia();

createApp(App)
  .use(router)
  .use(pinia)
  .component("c-button", Button)
  .component("c-configuration", Configuration)
  .component("c-input", Input)
  .mount("#app");
