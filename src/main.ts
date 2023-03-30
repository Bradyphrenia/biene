import { createApp } from "vue";

import "./main.less";

import Configuration from "./core/Configuration.vue";
import Input from "./core/Input.vue";

import App from "./App.vue";

import router from "./router";

createApp(App)
  .use(router)
  .component("c-configuration", Configuration)
  .component("c-input", Input)
  .mount("#app");
