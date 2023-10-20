import ReviewView from "./components/ReviewView.vue";
import SettingsView from "./components/SettingsView.vue";

import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: ReviewView,
  },
  {
    path: '/settings',
    component: SettingsView,
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
