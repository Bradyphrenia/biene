import { createApp } from "vue";
import { createPinia } from "pinia";

import "./main.less";

import Button from "./core/Button.vue";
import Configuration from "./core/Configuration.vue";
import Icon from "./core/Icon.vue";
import Input from "./core/Input.vue";

import App from "./App.vue";

import router from "./router";
import { icons } from "./util/icons.js";

const pinia = createPinia();

const app = createApp(App)
  .use(router)
  .use(pinia)
  .component("c-button", Button)
  .component("c-configuration", Configuration)
  .component("c-icon", Icon)
  .component("c-input", Input)

app.config.globalProperties.icons = icons
app.mount("#app");
